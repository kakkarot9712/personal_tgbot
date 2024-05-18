use std::fmt;

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::callback_query::traits::{KeyboardGenerator, QueryOptions};

#[derive(Clone, Copy)]
pub enum ButtonLayout {
    ListDues,
    AddPerson,
    AddTransaction,
}

impl fmt::Display for ButtonLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::ListDues => "List all Dues",
            Self::AddPerson => "Add a Peson",
            Self::AddTransaction => "Add a Transaction"
        };
        write!(f, "{}", name)
    }
}

impl QueryOptions for ButtonLayout {
    fn get_available_options() -> Vec<String> {
        vec![
            Self::ListDues.to_string(),
            Self::AddPerson.to_string(),
            Self::AddTransaction.to_string(),
        ]
    }

    fn has_query(q: &String) -> bool {
        let available_modes = Self::get_available_options();
        available_modes.contains(q)
    }
}

impl KeyboardGenerator for ButtonLayout {
    fn make_keyboard() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        let available_commands = Self::get_available_options();

        for commands in available_commands.chunks(2) {
            let row = commands
                .iter()
                .map(|cmd| InlineKeyboardButton::callback(cmd.to_owned(), cmd.to_owned()))
                .collect();

            keyboard.push(row);
        }
        InlineKeyboardMarkup::new(keyboard)
    }
}
