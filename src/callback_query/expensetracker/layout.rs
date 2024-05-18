use std::fmt;

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::callback_query::{
    traits::{GetAllOptions, KeyboardGenerator},
    HasQuery,
};

#[derive(Clone, Copy)]
pub enum ButtonLayout {
    ListDues,
    AddPerson,
    AddTransaction,
    SettleDues,
}

impl fmt::Display for ButtonLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::ListDues => "List all Dues",
            Self::AddPerson => "Add a Peson",
            Self::AddTransaction => "Add a Transaction",
            Self::SettleDues => "Settle Dues",
        };
        write!(f, "{}", name)
    }
}

impl GetAllOptions for ButtonLayout {
    fn get_available_options() -> Vec<String> {
        vec![
            Self::ListDues.to_string(),
            Self::AddPerson.to_string(),
            Self::AddTransaction.to_string(),
            Self::SettleDues.to_string(),
        ]
    }
}

impl HasQuery for ButtonLayout {
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
