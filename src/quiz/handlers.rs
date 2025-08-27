use console::StyledObject;
use console::style;
use dialoguer::Error;
use dialoguer::Input;
use dialoguer::Select;

use crate::config::Question;
use crate::config::WithCorrect;

use super::constants::*;

use console::Term;

fn create_styled_answer_status(correct: Option<bool>) -> StyledObject<&'static str> {
    match correct {
        Some(true) => style("✔ Correct").green().bold(),
        Some(false) => style("✖ Wrong").red().bold(),
        None => style("- N/A").white().bold(),
    }
}

pub fn handle_answer_result(
    instant_feedback: &bool,
    correct: Option<bool>,
    answers_vec: &mut Vec<Option<bool>>,
) {
    let stderr = Term::stderr();
    if *instant_feedback {
        stderr.write_line(&format!("{}", create_styled_answer_status(correct)));
    }

    stderr.write_line("");
    answers_vec.push(correct);
}

pub fn print_final_results(questions: &[Question], answers_vec: &[Option<bool>]) {
    let stdout = Term::stdout();
    for (i, value) in answers_vec.iter().enumerate() {
        let question: &str = questions[i].prompt.as_str();

        stdout.write_line(&format!(
            "{question}: {}",
            create_styled_answer_status(*value)
        ));
    }
}

pub fn handle_num_input(prompt: &str, correct: &Option<Vec<f64>>) -> Result<Option<bool>, Error> {
    let input: f64 = Input::new().with_prompt(prompt).interact()?;

    match correct {
        Some(vec) => Ok(Some(vec.contains(&input))),
        None => Ok(None),
    }
}

pub fn handle_text_input(
    prompt: &str,
    correct: &Option<Vec<String>>,
) -> Result<Option<bool>, Error> {
    let input: String = Input::new().with_prompt(prompt).interact()?;

    match correct {
        Some(vec) => Ok(Some(vec.contains(&input))),
        None => Ok(None),
    }
}

pub fn handle_truefalse(prompt: &str, correct: &bool) -> Result<Option<bool>, Error> {
    let options = [style("True").blue(), style("False").red()];

    let input = Select::new()
        .with_prompt(prompt)
        .items(options)
        .interact()?;

    Ok(Some(
        input
            == match correct {
                false => 1,
                true => 0,
            },
    ))
}

pub fn handle_single_choice(
    prompt: &str,
    options: &WithCorrect<Vec<String>>,
) -> Result<Option<bool>, Error> {
    let styles = get_answer_styles();
    let items: Vec<StyledObject<&String>> = options
        .answers
        .iter()
        .enumerate()
        .map(|(i, answer)| styles[i % styles.len()].apply_to(answer))
        .collect();

    let input = Select::new().with_prompt(prompt).items(&items).interact()? as u32;

    Ok(Some(options.correct.contains(&input)))
}
