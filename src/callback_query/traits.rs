use std::fmt::Display;

use teloxide::types::InlineKeyboardMarkup;

pub trait GetAllOptions
where
    Self: Display,
{
    fn get_available_options() -> Vec<String>;
}

pub trait HasQuery {
    fn has_query(q: &String) -> bool;
}

pub trait KeyboardGenerator
where
    Self: GetAllOptions,
{
    fn make_keyboard() -> InlineKeyboardMarkup;
}
