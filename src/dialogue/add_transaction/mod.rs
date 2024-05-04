use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub mod handler;

#[derive(Debug, Clone, Default)]
pub enum AddTransactionState {
    #[default]
    Idle,
    Started,
    AmountAsked { amount: i64 },
}

pub type AddTransactionDialogue = Dialogue<AddTransactionState, InMemStorage<AddTransactionState>>;
