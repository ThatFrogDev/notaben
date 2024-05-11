use crate::{
    api, multiselect,
    prompts::{
        confirm::confirm,
        input::{multi, single},
        select::select,
    },
    truncate_note,
    utilities::{cursor_to_origin::cursor_to_origin, display::display},
};
use async_std::path::PathBuf;
use chrono::prelude::Local;
use rusqlite::{params, Connection};
use std::io::{self, Write};

#[derive(Clone)]
pub struct Note {
    pub id: usize,
    pub name: String,
    pub content: String,
    pub created: String,
}

impl Note {
    pub fn create(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let sqlite = Connection::open(db_file)?;

        // Fetch IDs from database, sort and find the first gap
        // if it does not exist, use the length of the array + 1
        let mut stmt = sqlite.prepare("SELECT id FROM saved_notes")?;
        let ids: Result<Vec<usize>, _> = stmt.query_map(params![], |row| row.get(0))?.collect();
        let mut ids = ids?;
        ids.sort_unstable();
        let id = ids
            .clone()
            .into_iter()
            .enumerate()
            .find(|(i, id)| i + 1 != *id)
            .map_or_else(|| ids.len() + 1, |(i, _)| i + 1);

        cursor_to_origin()?;
        println!(
        "If you're done inputting a field, you can press Enter to continue or save, \n\
        For the \"Content\" field: to end a paragraph press Space and then Enter. To end your note, press Enter on a empty new line.\n\
        You can use Markdown to format your notes.\r"
    );

        let mut name: String;
        loop {
            name = single("Name:", "".to_string(), true)?;
            if name.len() > 64 {
                cursor_to_origin()?;
                println!(
                "If you're done inputting a field, you can press Enter twice to continue or save, or Alt/Option-Q to return to the main menu.\n\n\
                error: The name is too long, it must be 64 characters or less.\r"
            );
            } else {
                break;
            }
        }

        println!("Content:");
        let mut content = String::new();
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;

            // If the buffer is empty (only Enter was pressed), we consider it as end of input
            if buffer.trim().is_empty() {
                break;
            }

            content.push_str(&buffer);
        }

        let inputted_note = Note {
            id,
            name,
            content,
            created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
        };

        sqlite.execute(
            "INSERT INTO saved_notes (id, name, content, created) VALUES (?1, ?2, ?3, ?4);",
            params![
                &inputted_note.id,
                &inputted_note.name,
                &inputted_note.content,
                &inputted_note.created
            ],
        )?;
        cursor_to_origin()?;
        println!("Note created successfully.");
        Ok(())
    }

    pub fn show(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        cursor_to_origin()?;
        let saved_notes = api::get_notes(db_file)?;

        if saved_notes.is_empty() {
            println!("You don't have any notes.");
            return Ok(());
        }

        let mut options: Vec<String> = Vec::new();
        truncate_note(&mut options, db_file)?;
        let selection = select("Select the note that you want to view:", &options);
        let selected_note = &saved_notes[selection];
        cursor_to_origin()?;

        display(selected_note)?;
        Ok(())
    }

    pub fn edit(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        cursor_to_origin()?;
        let saved_notes = api::get_notes(db_file)?;

        if saved_notes.is_empty() {
            println!("You don't have any notes.");
            return Ok(());
        }

        println!(
        "If you're done editing a field, you can press Enter on an empty line to continue or save. \r"
    );
        let mut options: Vec<String> = Vec::new();
        truncate_note(&mut options, db_file)?;
        let selection = select("Select the note that you want to edit:", &options);
        let selected_note = &saved_notes[selection];

        let mut name: String;
        loop {
            name = single("Name:", selected_note.name.clone(), true)?;
            if name.len() > 64 {
                cursor_to_origin()?;
                println!(
                "If you're done inputting a field, you can press Enter twice to continue or save, or Alt/Option-Q to return to the main menu.\n\n\
                error: The name is too long, it must be 64 characters or less.\r"
            );
            } else {
                break;
            }
        }

        println!("{}", selected_note.content);

        println!("New Content:");
        let mut content = String::new();
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;

            // if nothing typed then break
            if buffer.trim().is_empty() {
                break;
            }

            content.push_str(&buffer);
        }

        let updated_note = Note {
            id: selected_note.id,
            name,
            content,
            created: selected_note.created.clone(),
        };

        cursor_to_origin()?;
        api::save_note(&updated_note, db_file)?; // data in my tummy
        println!("Note updated successfully.");
        Ok(())
    }

    pub fn delete(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        cursor_to_origin()?;
        // yep yep yeah
        if api::get_notes(db_file)?.is_empty() {
            println!("You can't delete notes, because there are none.");
            return Ok(());
        }

        let mut options: Vec<String> = Vec::new();
        truncate_note(&mut options, db_file)?;
        let selections = multiselect(
            "Select the note(s) that you want to delete:\nSpace to select, Enter to confirm.",
            options,
        );

        let mut prompt = "Are you sure that you want to delete these notes?";
        if selections.len() == 1 {
            prompt = "Are you sure that you want to delete this note?"; // i love this
        }

        cursor_to_origin()?;
        if selections.is_empty() {
            println!("You didn't select any notes.");
            Ok(())
        } else if confirm(prompt) {
            api::delete_notes(selections, db_file)?;
            //cursor_to_origin()?;
            println!("Notes deleted successfully.");
            Ok(())
        } else {
            cursor_to_origin()?;
            Ok(())
        }
    }
}
