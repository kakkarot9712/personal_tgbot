use std::sync::Arc;

use mongodb::Database;
use teloxide::prelude::*;

use crate::{
    database::{schema::Person, traits::CollectionHelpers},
    dialogue::{
        insert_cancel_helper_text,
        state::{DialogueWithState, State},
    },
    globals::Globals,
};

use super::types::ExpenseTrackerCommands;

pub async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: ExpenseTrackerCommands,
    db: Arc<Database>,
    dialogue: DialogueWithState,
    globals: Globals,
) -> ResponseResult<()> {
    match cmd {
        ExpenseTrackerCommands::AddPerson => {
            dialogue.update(State::PReceiveName).await.unwrap();
            bot.send_message(
                msg.chat.id,
                insert_cancel_helper_text("Okay! What is the Full Name of User?".to_owned()),
            )
            .await
            .unwrap();
        }
        ExpenseTrackerCommands::AddTransaction => {
            dialogue.update(State::TStarted).await.unwrap();
            bot.send_message(
                msg.chat.id,
                insert_cancel_helper_text("Okay! Enter the Amount.".to_owned()),
            )
            .await
            .unwrap();
        }
        ExpenseTrackerCommands::ListAllDues => {
            let mut formatted_msg = String::from("");
            let docs = Person::get_all(&db).await.unwrap();
            for person in docs.iter() {
                formatted_msg.push_str(&format!("{} :- {}\n", person.name, person.balance));
            }
            bot.send_message(msg.chat.id, "List of Dues by Persons:")
                .await
                .unwrap();
            if formatted_msg == "" {
                bot.send_message(
                    msg.chat.id,
                    "No Users found! Please Add New Person to get Started.",
                )
                .await
                .unwrap();
            } else {
                bot.send_message(msg.chat.id, formatted_msg).await.unwrap();
            }
        }
        ExpenseTrackerCommands::SettleDue => {
            let keyboard = Person::make_keyboard(db, false).await;
            dialogue.update(State::SDPersonAsked).await.unwrap();
            bot.send_message(
                msg.chat.id,
                insert_cancel_helper_text("Okay Select The Person:".to_owned()),
            )
            .reply_markup(keyboard)
            .await
            .unwrap();
        }
        ExpenseTrackerCommands::CheckAccessMode => {
            let globals_lock = globals.lock().await;
            let current_mode = globals_lock.get("allowAll");
            let mut is_allowed = false;
            match current_mode {
                Some(mode) => {
                    if mode == "true" {
                        is_allowed = true;
                    }
                }
                _ => {}
            }
            if is_allowed {
                bot.send_message(
                    msg.chat.id,
                    "Allow All: Anyone can Access Mode Specific Features.",
                )
                .await
                .unwrap();
            } else {
                bot.send_message(
                    msg.chat.id,
                    "Restricted: Only Owner can Access Mode Specific Features.",
                )
                .await
                .unwrap();
            }
        }
    }
    Ok(())
}
