use std::fmt;

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::callback_query::traits::{KeyboardGenerator, QueryOptions};

#[derive(Copy, Clone)]
pub enum ButtonLayout {
    Solo,
    SplitEq,
}

impl fmt::Display for ButtonLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Solo => "Single User",
            Self::SplitEq => "Multiple User",
        };
        write!(f, "{}", name)
    }
}

impl QueryOptions for ButtonLayout {
    fn get_available_options() -> Vec<String> {
        vec![Self::Solo.to_string(), Self::SplitEq.to_string()]
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
        for commands in available_commands.chunks(3) {
            let row = commands
                .iter()
                .map(|cmd| InlineKeyboardButton::callback(cmd.to_owned(), cmd.to_owned()))
                .collect();

            keyboard.push(row);
        }
        InlineKeyboardMarkup::new(keyboard)
    }
}
