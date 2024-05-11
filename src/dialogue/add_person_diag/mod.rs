pub mod handler;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum AddPersonDialogueState {
    #[default]
    Idle,
    Start,
    ReceiveName,
    ReceiveBalance {
        full_name: String,
    },
}
