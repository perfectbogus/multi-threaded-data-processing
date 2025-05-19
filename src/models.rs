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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_record_deserialization() {
        let json = r#"{
            "id": "001",
            "timestamp": "2023-01-01T12:00:00Z",
            "value": 100.5,
            "category": "A"
        }"#;
        
        let record: Record = serde_json::from_str(json).unwrap();

        assert_eq!(record.id, "001");
        assert_eq!(record.timestamp, "2023-01-01T12:00:00Z");
        assert_eq!(record.value, 100.5);
        assert_eq!(record.category, "A");
    }
    
    #[test]
    fn test_processed_record_serialization() {
        let processed = ProcessedRecord {
            id: "001".to_string(),
            normalized_value: 1.005,
            category: "A".to_string(),
            is_significant: true,
        };
        
        let json = serde_json::to_string(&processed).unwrap();
        assert!(json.contains("1.005"));
        assert!(json.contains("true"));
    }
}