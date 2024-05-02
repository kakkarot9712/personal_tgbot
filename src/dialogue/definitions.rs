use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[derive(Clone, Default, Debug)]
pub enum AddPersonDialogueState {
    #[default]
    Idle,
    Start,
    ReceiveName,
    ReceiveBalance {
        full_name: String,
    },
}

pub type MyDialogue = Dialogue<AddPersonDialogueState, InMemStorage<AddPersonDialogueState>>;
