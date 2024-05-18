pub mod handler;
use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum State {
    #[default]
    Idle,
    Start,
    ReceiveName,
    ReceiveBalance {
        full_name: String,
    },
}

pub type DialogueWithState = Dialogue<State, InMemStorage<State>>;
