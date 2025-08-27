#![allow(unused)]
use crate::config::Config;
use crate::config::QuestionType;

mod constants;
mod handlers;
use handlers::*;

use dialoguer::Error;

pub fn run_quiz(config: &Config) -> Result<(), Error> {
    let mut answers: Vec<Option<bool>> = Vec::new();

    for question in &config.questions {
        let result = match &question.options {
            QuestionType::SingleChoice(options) => handle_single_choice(&question.prompt, options)?,
            QuestionType::TrueFalse(correct) => handle_truefalse(&question.prompt, correct)?,
            QuestionType::TextInput(correct) => handle_text_input(&question.prompt, correct)?,
            QuestionType::NumInput(correct) => handle_num_input(&question.prompt, correct)?,
        };
        handle_answer_result(&config.instant_feedback, result, &mut answers);
    }

    print_final_results(&config.questions, &answers);

    Ok(())
}
