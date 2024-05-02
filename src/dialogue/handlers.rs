use super::definitions::{AddPersonDialogueState, MyDialogue};
use std::num::ParseIntError;
use teloxide::prelude::*;

pub async fn handle_due(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String,
    msg: Message,
) -> ResponseResult<()> {
    let balance: Result<u128, ParseIntError> = msg.text().unwrap().parse();
    match balance {
        Ok(num) => {
            bot.send_message(msg.chat.id, "Adding User to DB...")
                .await
                .unwrap();
            bot.send_message(
                msg.chat.id,
                format!("name: {}, balance: {}, id: {}", full_name, num, 5),
            )
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
