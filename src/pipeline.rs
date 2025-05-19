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
    }
}