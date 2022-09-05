use crate::repository::room::Repository;
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::Arc;

pub fn run<R: Repository>(repo: Arc<R>) {
    loop {
        let choices = [
            "Fetch all Pokemons",
            "Fetch a Pokemon",
            "Create a Pokemon",
            "Delete a Pokemon",
            "Exit",
        ];
        let index = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Make your choice")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(index) => index,
            _ => continue,
        };

        match index {
            4 => break,
            _ => continue,
        };
    }
}
