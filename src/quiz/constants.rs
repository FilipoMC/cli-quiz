use console::Style;

pub fn get_answer_styles() -> Vec<Style> {
    vec![
        Style::new().blue(),
        Style::new().red(),
        Style::new().yellow(),
        Style::new().green(),
        Style::new().magenta(),
        Style::new().cyan(),
    ]
}
