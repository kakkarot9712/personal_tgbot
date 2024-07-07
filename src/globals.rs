use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

pub type Globals = Arc<Mutex<HashMap<String, String>>>;
