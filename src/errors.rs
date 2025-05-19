use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Failed to read input file: {0}")]
    InputError(#[from] std::io::Error),
    
    #[error("Failed to parse data: {0}")]
    ParseError(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("Output error: {0}")]
    OutputError(String)
}