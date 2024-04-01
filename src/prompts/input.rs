use crate::{main, return_to_main};
use dialoguer::{theme::ColorfulTheme, Input};

pub fn single(prompt: &str, initial_text: String, is_required: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String;

    loop {
        result = match initial_text.as_str() {
            "" => Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .interact_text()
                .unwrap(),
            _ => Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .with_initial_text(initial_text.clone())
                .interact_text()
                .unwrap(),
        };

        if !is_required || !result.trim().is_empty() {
            break;
        }
    }

    match return_to_main() {
        Ok(value) => {
            if value == "userReturned" {
                main()?;
            } else if value == "userContinued" {
                return Ok(result);
            }
        }
        _ => return Ok(result),
    }

    Ok(result)
}

pub fn multi(prompt: &str, initial_text: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    let mut t_enter = 0;
    let mut first_iteration = true;

    loop {
        let line = if first_iteration {
            first_iteration = false;
            single(prompt, initial_text.clone(), false)?
        } else {
            single("", initial_text.clone(), false)?
        };

        if line.trim().is_empty() {
            t_enter += 1;
            if t_enter == 2 {
                break;
            }
        } else {
            t_enter = 0;
        }
        result.push_str(&line);
        result.push('\n');
    }

    Ok(result)
}