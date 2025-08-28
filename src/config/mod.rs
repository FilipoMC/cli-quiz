pub mod errors;
pub use errors::*;

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct WithCorrect<T> {
    pub answers: T,
    pub correct: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub enum CaseSensitivity<T> {
    CaseSens(T),
    NotCaseSens(T),
}

#[derive(Deserialize, Debug)]
pub enum QuestionType {
    SingleChoice(WithCorrect<Vec<String>>),
    TrueFalse(bool),
    TextInput(Option<CaseSensitivity<Vec<String>>>),
    NumInput(Option<Vec<f64>>),
}

#[derive(Deserialize, Debug)]
pub struct Question {
    pub prompt: String,
    pub options: QuestionType,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub questions: Vec<Question>,
    pub instant_feedback: bool,
}

fn find_path(cli_arg: &Option<PathBuf>) -> PathBuf {
    if let Some(path) = cli_arg {
        return path.clone();
    }

    if let Ok(exe_path) = env::current_exe()
        && let Some(bin_dir) = exe_path.parent()
    {
        let candidate = bin_dir.join("config.ron");
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from("config.ron")
}

pub fn load_config(cli_arg: &Option<PathBuf>) -> ConfigResult {
    let contents = fs::read_to_string(find_path(cli_arg))?;

    let config: Config = ron::from_str(&contents)?;
    Ok(config)
}
