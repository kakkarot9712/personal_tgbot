use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub mod handler;
pub mod split;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum State {
    #[default]
    Idle,
    Started,
    AmountAsked {
        amount: f64,
    },
}

pub type DialogueWithState = Dialogue<State, InMemStorage<State>>;
