use crate::{
    commands::{self, types::*},
    dialogue::{add_person_diag, add_transaction_diag, mode::ModeState, settle_due, state::State},
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
    let mut current_userid: Option<String> = None;
    let is_me = match sender {
        Some(u) => {
            current_userid = Some(u.id.to_string());
            u.id.to_string() == my_id.to_string()
        }
        None => false,
    };
    if !is_me {
        bot.send_message(msg.chat.id, format!("Only owner can use mode specific features of this bot as of now. If you want to use bot, kindly goto Github source code of this bot and follow README.md to deploy your own verson of this bot. use /source command to get source code of this bot. Your user ID is {}",current_userid.unwrap_or("NOT_FOUND".to_owned()))).await.unwrap();
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
    let command_handler = dptree::entry().branch(
        dptree::case![State::Idle]
            .branch(
                dptree::entry()
                    .filter_command::<SimpleCommands>()
                    .endpoint(commands::simple::handle_commands),
            )
            .filter_async(only_me)
            .branch(
                dptree::case![ModeState::ExpenseTracker]
                    .filter_command::<ExpenseTrackerCommands>()
                    .endpoint(commands::expensetrackercommands::handle_commands),
            ),
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
        .branch(dialogue_handler)
        .branch(hidden_command_handler)
        .branch(command_handler);

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            dptree::case![State::TNoteAsked {
                amount,
                note,
                persons
            }]
            .endpoint(add_transaction_diag::handle_callback_query),
        )
        .branch(dptree::case![State::SDPersonAsked].endpoint(settle_due::handle_person_asked));

    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .enter_dialogue::<Update, InMemStorage<ModeState>, ModeState>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
