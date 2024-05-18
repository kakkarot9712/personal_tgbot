use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tgbot::dialogue::add_transaction_diag::split;
use tgbot::dialogue::settle_due;
use tgbot::schema::schema;
use tgbot::{database::initialize_db, dialogue::add_person_diag};

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
            InMemStorage::<add_person_diag::State>::new(),
            InMemStorage::<split::State>::new(),
            InMemStorage::<settle_due::State>::new(),
            db
        ])
        .build()
        .dispatch()
        .await;
}
