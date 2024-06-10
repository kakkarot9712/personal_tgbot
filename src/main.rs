use std::collections::HashMap;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tgbot::dialogue::state::State;
use tgbot::schema::schema;
use tgbot::database::initialize_db;
use tgbot::user_state::{UserState, UserStateMapping};
use tokio::sync::Mutex;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    println!("Starting bot...");

    let db = Arc::new(initialize_db().await.expect("DB Connection Failed!"));
    let hashmap: UserStateMapping = Arc::new(Mutex::new(HashMap::<String, UserState>::new()));
    let bot = Bot::new(dotenv!("BOT_TOKEN"));

    Dispatcher::builder(bot, schema())
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            db,
            hashmap
        ])
        .build()
        .dispatch()
        .await;
}
