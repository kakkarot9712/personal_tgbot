use dotenv_codegen::dotenv;
use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    dialogue::state::State,
    globals::Globals,
    user_state::{UserState, UserStateMapping},
};

pub async fn only_me(
    msg: Message,
    bot: Bot,
    user_state: UserStateMapping,
    globals: Globals,
) -> bool {
    let my_id = dotenv!("MYID");
    let sender = msg.from();
    let mut current_userid: Option<String> = None;
    let globals_lock = globals.lock().await;
    let mut is_me = match sender {
        Some(u) => {
            current_userid = Some(u.id.to_string());
            u.id.to_string() == my_id.to_string()
        }
        None => false,
    };
    if !is_me {
        let access_mode = globals_lock.get("allowAll");
        if let Some(mode) = access_mode {
            if mode == "true" {
                is_me = true;
            }
        }
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
    if !is_me {
        bot.send_message(msg.chat.id, format!("Only owner can use mode specific features of this bot as of now. If you want to use bot, kindly goto Github source code of this bot and follow README.md to deploy your own verson of this bot. use /source command to get source code of this bot. Your user ID is {}",current_userid.unwrap_or("NOT_FOUND".to_owned()))).await.unwrap();
    }
    is_me
}
