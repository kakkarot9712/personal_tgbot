use std::fmt;

use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

use crate::database::schema::Person;

#[derive(Clone, Default, Debug)]
pub enum State {
    // add_person
    #[default]
    Idle,
    PStart,
    PReceiveName,
    PReceiveBalance {
        full_name: String,
    },
    // Settle_due
    SDPersonAsked,
    SDAmountAsked {
        person: Person,
    },

    // add-transaction_split
    TStarted,
    TAmountAsked {
        amount: i64,
    },
    TNoteAsked {
        amount: i64,
        note: String,
        persons: Option<Vec<Person>>,
    },
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: Option<&str>;
        match self {
            State::TAmountAsked { amount: _ } => name = Some("T_AmountAsked"),
            State::Idle => name = Some("idle"),
            State::TNoteAsked {
                amount: _,
                note: _,
                persons: _,
            } => name = Some("T_NoteAsked"),
            State::TStarted => name = Some("T_Started"),
            State::PStart => name = Some("P_Start"),
            State::PReceiveName => name = Some("P_ReceiveName"),
            State::PReceiveBalance {full_name: _}  => name = Some("P_ReceiveBalance"),
            State::SDPersonAsked => name = Some("SD_PersonAsked"),
            State::SDAmountAsked { person:_ } => name = Some("SD_AmountAsked")
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
