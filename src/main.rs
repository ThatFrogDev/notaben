// TODO: Organize the code into different files
pub mod api;

use chrono::prelude::*;
use inquire::{Confirm, Select, Text};

pub struct Note {
    id: usize,
    name: String,
    content: String,
    created: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::init_db()?;
    println!("Welcome to Notabena, the FOSS note taking app.");
    println!("=======================");

    loop {
        cursor_to_origin();

        let options: Vec<&str> = vec!["New note", "View notes", "Edit note", "Delete note", "Exit"];
        let select = Select::new("What do you want to do?", options).prompt();

        match select {
            Ok("New note") => {
                new_note().expect("Creating a new note failed");
                cursor_to_origin();
            },
            Ok("View notes") => {
                show_notes()?;
                cursor_to_origin();
            },
            Ok("Edit note") => {
                edit_notes().expect("Editing the note failed");
                cursor_to_origin();
            },
            Ok("Delete note") => {
                delete_notes().expect("Deleting the note failed");
                cursor_to_origin();
            },
            Ok(_) => {
                return Ok(());
            },
            Err(e) => {
                println!("There was an error: {}", e);
            },
        }
    }
}

fn new_note() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;

    let inputted_note = Note {
        id: saved_notes.len(),
        name: Text::new("Name:").prompt()?,
        content: Text::new("Content:").prompt()?,
        created: format!("{}", Local::now().format("%A %e %B, %H:%M").to_string()),
    };

    cursor_to_origin();
    println!("This is the note you just created:");
    println!("=======================");
    println!("Name: {}", inputted_note.name);
    println!("Content: {}", inputted_note.content);
    println!("Created at: {}", inputted_note.created);
    println!("=======================");

    let save_note_bool = Confirm::new("Do you want to save this note?")
        .with_default(true)
        .prompt();

    return match save_note_bool {
        Ok(true) => {
            api::save_note(&inputted_note)?;
            Ok(())
        }
        Ok(false) => Ok(()),
        Err(e) => {
            println!("There was an error: {}", e);
            Err(Box::new(e))
        }
    };
}

fn show_notes() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Make the notes selectable to fully view them
    let saved_notes = api::get_notes()?;

    if saved_notes.is_empty() {
        println!("You don't have any notes.");
        return Ok(());
    } else {
        for note in saved_notes {
            println!("{}", note.name);
            println!("{}", note.content);
            println!("Created at: {}", note.created);
            println!("=======================");
        }
        return Ok(());
    };
}

fn edit_notes() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let mut options: Vec<String> = Vec::new();

    if saved_notes.is_empty() {
        println!("You can't edit notes, because there are none.");
        Ok(())
    } else { Ok({
        for note in &saved_notes {
            let mut truncated_content: String = note
                .content
                .chars()
                .take(10)
                .collect();

            if truncated_content.chars().count() == 10 {
                truncated_content = truncated_content + "...";
            }

            options.push(format!("{} | {} | {}", note.name, truncated_content, note.created));
        }

        let selection = Select::new("Select the note that you want to edit: ", options.clone()).prompt();
        let selection_index = options.iter().position(|n| n == selection.as_ref().unwrap()).unwrap();

        match selection_index {
            index => {
                let selected_note = &saved_notes[index];
                let edited_name = Text::new("New name:")
                    .with_initial_value(&selected_note.name)
                    .prompt()?;
                let edited_content = Text::new("New content:")
                    .with_initial_value(&selected_note.content)
                    .prompt()?;

                let updated_note = Note {
                    id: index,
                    name: edited_name,
                    content: edited_content,
                    created: selected_note.created.clone(),
                };

                api::edit_note(&updated_note, index)?;
                println!("Note updated successfully.");
            }
        }
    }) }
}

fn delete_notes() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement select multiple and delete all
    let saved_notes = api::get_notes()?;
    let mut options: Vec<String> = Vec::new();

    if saved_notes.is_empty() {
        println!("You can't delete notes, because there are none.");
        Ok(())
    } else { Ok({
        for note in &saved_notes {
            let mut truncated_content: String = note
                .content
                .chars()
                .take(10)
                .collect();

            if truncated_content.chars().count() == 10 {
                truncated_content = truncated_content + "...";
            }

            options.push(format!("{} | {} | {}", note.name, truncated_content, note.created));
        }

        let selection = Select::new("Select the note that you want to delete: ", options.clone()).prompt();
        let selection_index = options.iter().position(|n| n == selection.as_ref().unwrap()).unwrap();

        match selection_index {
            index => {
                api::delete_note(index)?;
                println!("Note deleted successfully.");
            }
        }
    }) }
}

fn cursor_to_origin() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
