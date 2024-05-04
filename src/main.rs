use std::sync::Arc;

use expensetrackerbot::commands;
use expensetrackerbot::dialogue::add_transaction::{self, AddTransactionState};
use expensetrackerbot::{
    commands::definitions::SimpleCommands,
    db::initialize_db,
    dialogue::add_person_diag::{handler, AddPersonDialogueState},
};
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
    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<AddPersonDialogueState>, AddPersonDialogueState>()
        .enter_dialogue::<Message, InMemStorage<AddTransactionState>, AddTransactionState>()
        .branch(
            dptree::case![AddPersonDialogueState::Idle]
                .filter_command::<SimpleCommands>()
                .endpoint(commands::handlers::handle_commands),
        )
        .branch(dptree::case![AddPersonDialogueState::ReceiveName].endpoint(handler::handle_name))
        .branch(
            dptree::case![AddPersonDialogueState::ReceiveBalance { full_name }]
                .endpoint(handler::handle_due),
        )
        .branch(
            dptree::case![AddTransactionState::Started].endpoint(add_transaction::handler::start),
        )
        .branch(
            dptree::case![AddTransactionState::AmountAsked { amount }]
                .endpoint(add_transaction::handler::handle_amount_asked),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![
            InMemStorage::<AddPersonDialogueState>::new(),
            InMemStorage::<AddTransactionState>::new(),
            db
        ])
        .build()
        .dispatch()
        .await;
}
