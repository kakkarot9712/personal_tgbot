use crate::{
    callback_query,
    commands::{self, types::*},
    dialogue::{add_person_diag, add_transaction_diag, settle_due, state::State},
    user_state::{UserState, UserStateMapping},
};
use dotenv_codegen::dotenv;
use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, di::DependencyMap, Handler},
    prelude::*,
    types::{Message, Update},
    Bot, RequestError,
};

async fn only_me(msg: Message, bot: Bot, user_state: UserStateMapping) -> bool {
    let my_id = dotenv!("MYID");
    let sender = msg.from();
    let is_me = match sender {
        Some(u) => u.id.to_string() == my_id.to_string(),
        None => false,
    };
    if !is_me {
        bot.send_message(msg.chat.id, "Only owner can use this bot as of now. If you want to use bot, kindly goto Github profile and follow deployment steps").await.unwrap();
    } else {
        let chat_id = msg.chat.id.to_string();
        let mut user_state_lock = user_state.lock().await;
        let current_user = user_state_lock.get(&chat_id);
        let _ = match current_user {
            None => {
                let new_state = UserState {
                    dialogue_state: State::Idle,
                    user_id: my_id.to_string(),
                };
                user_state_lock.insert(chat_id, new_state);
            }
            Some(_) => {
                // User already exists in hashmap
                // println!("{:?}", u);
            }
        };
    }
    is_me
}

pub fn schema() -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    let command_handler = teloxide::filter_command::<SimpleCommands, _>().branch(
        dptree::case![State::Idle]
            .filter_command::<SimpleCommands>()
            .endpoint(commands::simple::handle_commands),
    );

    let hidden_command_handler = dptree::entry()
        .filter_command::<HiddenCommands>()
        .endpoint(HiddenCommands::handle_commands);

    let dialogue_handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<HiddenCommands>()
                .endpoint(HiddenCommands::handle_commands),
        )
        .branch(dptree::case![State::PReceiveName].endpoint(add_person_diag::handle_name))
        .branch(
            dptree::case![State::PReceiveBalance { full_name }]
                .endpoint(add_person_diag::handle_due),
        )
        .branch(dptree::case![State::TStarted].endpoint(add_transaction_diag::start))
        .branch(
            dptree::case![State::TAmountAsked { amount }]
                .endpoint(add_transaction_diag::handle_amount_asked),
        )
        .branch(
            dptree::case![State::SDAmountAsked { person }]
                .endpoint(settle_due::handle_amount_asked),
        );

    let message_handler = Update::filter_message()
        .filter_async(only_me)
        .branch(command_handler)
        .branch(dialogue_handler)
        .branch(hidden_command_handler);

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            dptree::case![State::TNoteAsked {
                amount,
                note,
                persons
            }]
            .endpoint(add_transaction_diag::handle_callback_query),
        )
        .branch(dptree::case![State::SDPersonAsked].endpoint(settle_due::handle_person_asked))
        .branch(dptree::endpoint(callback_query::handle_callback));

    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
