pub mod handler;

use std::fmt;

use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

use crate::database::schema::Person;

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Idle,
    PersonAsked,
    AmountAsked {
        person: Person,
    },
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Idle => "Idle",
            Self::AmountAsked { person: _ } => "AmountAsked",
            Self::PersonAsked => "PersonAsked",
        };
        write!(f, "{}", name)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }

    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }
}

pub type DialogueWithState = Dialogue<State, InMemStorage<State>>;
