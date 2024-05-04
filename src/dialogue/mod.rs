pub mod add_person_diag;
pub mod add_transaction;

use add_person_diag::AddPersonDialogueState;
use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub type MyDialogue = Dialogue<AddPersonDialogueState, InMemStorage<AddPersonDialogueState>>;
