use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::dialogue::state::State;

#[derive(Debug)]
pub struct UserState {
    pub dialogue_state: State,
    pub user_id: String
}

pub type UserStateMapping = Arc<Mutex<HashMap<String, UserState>>>;
