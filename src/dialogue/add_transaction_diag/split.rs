use std::fmt;

use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

use crate::database::schema::Person;

pub mod handler;

#[derive(Debug, Clone, Default)]
pub enum State {
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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: Option<&str>;
        match self {
            State::AmountAsked { amount: _ } => name = Some("AmountAsked"),
            State::Idle => name = Some("idle"),
            State::NoteAsked {
                amount: _,
                note: _,
                persons: _,
            } => name = Some("NoteAsked"),
            State::Started => name = Some("Started"),
        }
        write!(f, "{}", name.unwrap())
    }
}

impl PartialEq for State {
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

pub type DialogueWithState = Dialogue<State, InMemStorage<State>>;
