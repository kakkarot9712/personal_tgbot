use expensetrackerbot::dialogue::add_transaction::{
    split::AddSplitTransactionState, AddTransactionState,
};
use expensetrackerbot::schema::schema;
use expensetrackerbot::{db::initialize_db, dialogue::add_person_diag::AddPersonDialogueState};
use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    println!("Starting bot...");

    let db = Arc::new(initialize_db().await.expect("DB Connection Failed!"));

    let bot = Bot::new(dotenv!("BOT_TOKEN"));

    Dispatcher::builder(bot, schema())
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![
            InMemStorage::<AddPersonDialogueState>::new(),
            InMemStorage::<AddTransactionState>::new(),
            InMemStorage::<AddSplitTransactionState>::new(),
            db
        ])
        .build()
        .dispatch()
        .await;
}
