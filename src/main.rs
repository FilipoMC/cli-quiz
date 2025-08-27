mod cli;
mod config;
mod quiz;

use cli::Args;
use config::Config;

use clap::Parser;
use console::Term;

use std::process;

fn main() {
    let stderr = Term::stderr();

    let args: Args = Args::parse();

    let config: Config = match config::load_config(&args.config) {
        Ok(config) => config,
        Err(e) => {
            stderr.write_line(&format!("{}", e)).unwrap();
            process::exit(1);
        }
    };

    if let Err(e) = quiz::run_quiz(&config) {
        stderr
            .write_line(&format!("There was an error when running the quiz: {}", e))
            .unwrap();
        process::exit(1);
    }
}
