use crate::errors::PipelineError;
use crate::models::{PipelineConfig, Record, ProcessedRecord};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub struct Pipeline {
    config: PipelineConfig,
}

impl Pipeline {
    pub fn new(config: PipelineConfig) -> Self {
        Pipeline { config }
    }
    
    fn read_csv(&self, reader: BufReader<File>) -> Result<Vec<Record>, PipelineError> {
        let mut csv_reader = csv::Reader::from_reader(reader);
        let records: Result<Vec<Record>, _> = csv_reader
            .deserialize()
            .collect();
        
        records.map_err(|e| PipelineError::ParseError(e.to_string()))
    }
    
    fn read_json(&self, reader: BufReader<File>) -> Result<Vec<Record>, PipelineError> {
        let records: Vec<Record> = serde_json::from_reader(reader)
            .map_err(|e| PipelineError::ParseError(e.to_string()))?;
        
        Ok(records)
    }
    
    fn process_record(&self, record: Record) -> Result<ProcessedRecord, PipelineError> {
        let normalized_value = record.value / self.config.normalization_factor;
        let is_significant = normalized_value > self.config.significance_threshold;
        
        Ok(ProcessedRecord {
            id: record.id,
            normalized_value,
            category: record.category,
            is_significant,
        })
    }
    
    fn process_data(&self, records: Vec<Record>) -> Result<Vec<ProcessedRecord>, PipelineError> {
        let processed: Vec<ProcessedRecord> = records
            .into_iter()
            .map(|record| self.process_record(record))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(processed)
    }
    
    fn write_csv(&self, writer: BufWriter<File>, records: &[ProcessedRecord]) -> Result<(), PipelineError> {
        let mut csv_writer = csv::Writer::from_writer(writer);
        
        for record in records {
            csv_writer
                .serialize(record)
                .map_err(|e| PipelineError::OutputError(e.to_string()))?;
        }
        
        csv_writer.flush().map_err(|e| PipelineError::OutputError(e.to_string()))?;
        Ok(())
    }
    
    fn write_json(&self, writer: BufWriter<File>, records: &[ProcessedRecord]) -> Result<(), PipelineError> {
        serde_json::to_writer_pretty(writer, records)
            .map_err(|e| PipelineError::OutputError(e.to_string()))?;
        
        Ok(())
    }
    
    fn write_output(&self, records: &[ProcessedRecord]) -> Result<(), PipelineError> {
        let path = Path::new(&self.config.output_path);
        let file = File::create(path).map_err(|e| PipelineError::OutputError(e.to_string()))?;
        let writer = BufWriter::new(file);
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("csv") {
            self.write_csv(writer,records)
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            self.write_json(writer, records)
        } else {
            Err(PipelineError::OutputError("Unsupported output format".to_string()))
        }
    }
    
    fn read_input(&self) -> Result<Vec<Record>, PipelineError> {
        let path = Path::new(&self.config.input_path);
        let file = File::open(path).map_err(PipelineError::InputError)?;
        let reader = BufReader::new(file);
        
        // Determine file type and parse accordingly
        if path.extension().and_then(|ext| ext.to_str()) == Some("csv") {
            self.read_csv(reader)
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("json") { 
            self.read_json(reader)
        } else {
            Err(PipelineError::ParseError("Unsupported file format".to_string()))
        }
    }
    
    pub fn run(&self) -> Result<(), PipelineError> {
        println!("Starting pipeline");
        
        let records = self.read_input()?;
        println!("Read {} records", records.len());
        
        let processed_records = self.process_data(records)?;
        println!("Processed {} records", processed_records.len());
        
        self.write_output(&processed_records)?;
        println!("Pipeline completed successfully");

        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PipelineConfig, Record};

    fn create_test_csv_config() -> PipelineConfig {
        PipelineConfig {
            input_path: "test_input.csv".to_string(),
            output_path: "test_output.csv".to_string(),
            normalization_factor: 100.0,
            significance_threshold: 0.9,
        }
    }

    fn create_test_json_config() -> PipelineConfig {
        PipelineConfig {
            input_path: "test_input.json".to_string(),
            output_path: "test_output.json".to_string(),
            normalization_factor: 100.0,
            significance_threshold: 0.9,
        }
    }

    #[test]
    fn test_process_record() {
        let config = create_test_csv_config();
        let pipeline = Pipeline::new(config);

        let record = Record {
            id: "001".to_string(),
            timestamp: "2023-01-01T12:00:00Z".to_string(),
            value: 100.0,
            category: "A".to_string(),
        };

        let processed = pipeline.process_record(record).unwrap();

        assert_eq!(processed.id, "001");
        assert_eq!(processed.normalized_value, 1.0);
        assert_eq!(processed.category, "A");
        assert_eq!(processed.is_significant, true);
    }

    #[test]
    fn test_read_csv() {
        let pipeline = Pipeline::new(create_test_csv_config());
        let path = Path::new(&pipeline.config.input_path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let records = pipeline.read_csv(reader).unwrap();

        assert_eq!(records.len(), 5);
        assert_eq!(records[0].id, "001");
        assert_eq!(records[1].id, "002");
    }

    #[test]
    fn test_read_json() {
        let pipeline = Pipeline::new(create_test_json_config());
        let path = Path::new(&pipeline.config.input_path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file); 
        // let records = pipeline.read_csv(reader).unwrap();
    
        // assert_eq!(records.len(), 2);
        // assert_eq!(records[0].id, "001");
        // assert_eq!(records[1].id, "002");
    }
}