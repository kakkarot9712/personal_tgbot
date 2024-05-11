use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub mod handler;
pub mod split;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AddTransactionState {
    #[default]
    Idle,
    Started,
    AmountAsked {
        amount: i64,
    },
}

pub type AddTransactionDialogue = Dialogue<AddTransactionState, InMemStorage<AddTransactionState>>;
