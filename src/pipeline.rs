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