use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub id: String,
    pub timestamp: String,
    pub value: f64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessedRecord {
    pub id: String,
    pub normalized_value: f64,
    pub category: String,
    pub is_significant: bool,
}

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub input_path: String,
    pub output_path: String,
    pub normalization_factor: f64,
    pub significance_threshold: f64,
}