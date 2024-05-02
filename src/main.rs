use expensetrackerbot::commands;
use expensetrackerbot::{
    commands::definitions::SimpleCommands,
    dialogue::{definitions::AddPersonDialogueState, handlers},
};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    println!("Starting bot...");

    let bot = Bot::new(dotenv!("BOT_TOKEN"));
    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<AddPersonDialogueState>, AddPersonDialogueState>()
        .branch(
            dptree::case![AddPersonDialogueState::Idle]
                .filter_command::<SimpleCommands>()
                .endpoint(commands::handlers::handle_commands),
        )
        .branch(dptree::case![AddPersonDialogueState::ReceiveName].endpoint(handlers::handle_name))
        .branch(
            dptree::case![AddPersonDialogueState::ReceiveBalance { full_name }]
                .endpoint(handlers::handle_due),
        );
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![InMemStorage::<AddPersonDialogueState>::new()])
        .build()
        .dispatch()
        .await;
}
