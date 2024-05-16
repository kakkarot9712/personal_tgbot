use crate::{
    db::{collections::Person, DBHandle},
    dialogue::MyDialogue,
};
use mongodb::Database;
use std::{num::ParseFloatError, sync::Arc};
use teloxide::prelude::*;

use super::AddPersonDialogueState;

pub async fn handle_due(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String,
    msg: Message,
    db: Arc<Database>,
) -> ResponseResult<()> {
    let balance: Result<f64, ParseFloatError> = msg.text().unwrap().parse();
    match balance {
        Ok(num) => {
            bot.send_message(msg.chat.id, "Adding User to DB...")
                .await
                .unwrap();
            let col_handle = Person::get_collection_handle(&db);
            col_handle
                .insert_one(
                    Person {
                        id: None,
                        name: full_name,
                        balance: num,
                    },
                    None,
                )
                .await
                .unwrap();
            bot.send_message(msg.chat.id, format!("Insert Success!"))
                .await
                .unwrap();
            dialogue.update(AddPersonDialogueState::Idle).await.unwrap();
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "Please send positive Number!")
                .await
                .unwrap();
        }
    }
    Ok(())
}

pub async fn handle_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> ResponseResult<()> {
    let full_name = msg.text().unwrap().to_string();
    dialogue
        .update(AddPersonDialogueState::ReceiveBalance { full_name })
        .await
        .unwrap();
    bot.send_message(msg.chat.id, "What's Current Due of User?")
        .await?;
    Ok(())
}
