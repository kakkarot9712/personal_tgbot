use std::fmt::Display;

use teloxide::types::InlineKeyboardMarkup;

pub trait QueryOptions
where
    Self: Display,
{
    fn get_available_options() -> Vec<String>;
    fn has_query(q: &String) -> bool;
}

pub trait KeyboardGenerator
where
    Self: QueryOptions,
{
    fn make_keyboard() -> InlineKeyboardMarkup;
}
