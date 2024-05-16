use std::fmt;

use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

use crate::db::collections::Person;

pub mod handler;

#[derive(Debug, Clone, Default)]
pub enum AddSplitTransactionState {
    #[default]
    Idle,
    Started,
    AmountAsked {
        amount: i64,
    },
    NoteAsked {
        amount: i64,
        note: String,
        persons: Option<Vec<Person>>,
    },
}

impl fmt::Display for AddSplitTransactionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: Option<&str>;
        match self {
            AddSplitTransactionState::AmountAsked { amount: _ } => name = Some("AmountAsked"),
            AddSplitTransactionState::Idle => name = Some("idle"),
            AddSplitTransactionState::NoteAsked {
                amount: _,
                note: _,
                persons: _,
            } => name = Some("NoteAsked"),
            AddSplitTransactionState::Started => name = Some("Started"),
        }
        write!(f, "{}", name.unwrap())
    }
}

impl PartialEq for AddSplitTransactionState {
    fn eq(&self, other: &Self) -> bool {
        if self.to_string() == other.to_string() {
            true
        } else {
            false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        if !Self::eq(&self, other) {
            true
        } else {
            false
        }
    }
}

pub type AddSplitTransactionDialogue =
    Dialogue<AddSplitTransactionState, InMemStorage<AddSplitTransactionState>>;
