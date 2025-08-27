use clap::Parser;
use std::path::PathBuf;

/// Simple CLI quiz app
#[derive(Parser, Debug)]
#[command(author = "filipo", version = "0.0.1", about = "CLI quiz example")]
pub struct Args {
    /// Config file path
    #[arg(long, short)]
    pub config: Option<PathBuf>,
}
