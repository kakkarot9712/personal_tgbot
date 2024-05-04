pub mod handler;

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
