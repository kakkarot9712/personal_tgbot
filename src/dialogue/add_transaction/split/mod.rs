use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub mod handler;

#[derive(Debug, Clone, Default, PartialEq)]
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
    },
}

pub type AddSplitTransactionDialogue =
    Dialogue<AddSplitTransactionState, InMemStorage<AddSplitTransactionState>>;
