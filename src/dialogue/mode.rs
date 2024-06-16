use std::fmt;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::Dialogue};

#[derive(Clone, Default, Debug)]
pub enum ModeState {
    #[default]
    ExpenseTracker,
}

impl fmt::Display for ModeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: Option<&str>;
        match self {
            ModeState::ExpenseTracker => name = Some("ExpenseTracker"),
        }
        write!(f, "{}", name.unwrap())
    }
}

pub type DialogueWithModeState = Dialogue<ModeState, InMemStorage<ModeState>>;
