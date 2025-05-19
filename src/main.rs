mod models;
mod errors;
mod pipeline;

use clap::Parser;
use models::PipelineConfig;
use pipeline::Pipeline;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value_t = 1.0)]
    normalization: f64,

    #[arg(short, long, default_value_t = 0.5)]
    threshold: f64,
}

fn main() {
    let args = Args::parse();
    
    let config = PipelineConfig {
        input_path: args.input,
        output_path: args.output,
        normalization_factor: args.normalization,
        significance_threshold: args.threshold,
    };
    
    let pipeline = Pipeline::new(config);
    
    if let Err(e) = pipeline.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
}