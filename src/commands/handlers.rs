use std::sync::Arc;

use mongodb::Database;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    // db::{pull_persons_data, Person},
    db::collections::Person,
    dialogue::{add_person_diag::AddPersonDialogueState, add_transaction::{AddTransactionDialogue, AddTransactionState}, MyDialogue},
};

use super::definitions::SimpleCommands;

pub async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: SimpleCommands,
    dialogue: MyDialogue,
    add_transaction_diag: AddTransactionDialogue,
    db: Arc<Database>,
) -> ResponseResult<()> {
    match cmd {
        SimpleCommands::Ping => {
            bot.send_message(msg.chat.id, "PONG").await.unwrap();
        }
        SimpleCommands::Help => {
            bot.send_message(msg.chat.id, SimpleCommands::descriptions().to_string())
                .await
                .unwrap();
        }
        SimpleCommands::ListDues => {
            let mut formatted_msg = String::new();
            let col = Person::get_collection_handle(&db);
            let mut cursor = col.find(None, None).await.unwrap();
            loop {
                match cursor.advance().await {
                    Ok(r) => {
                        if !r {
                            break;
                        }
                    }
                    Err(_) => {
                        bot.send_message(msg.chat.id, "DB Operation Failed!")
                            .await
                            .unwrap();
                        break;
                    }
                };
                let person = cursor.deserialize_current().unwrap();
                formatted_msg.push_str(&format!("{} :- {}\n", person.name, person.balance));
            }
            bot.send_message(msg.chat.id, formatted_msg).await.unwrap();
        }
        SimpleCommands::AddPerson => {
            dialogue
                .update(AddPersonDialogueState::ReceiveName)
                .await
                .unwrap();
            bot.send_message(msg.chat.id, format!("Okay! What is the Full Name of User?"))
                .await
                .unwrap();
        }
        SimpleCommands::AddTransaction => {
            add_transaction_diag.update(AddTransactionState::Started).await.unwrap();
            bot.send_message(msg.chat.id, "Okay! Enter the Amount.").await.unwrap();
        }
    };
    Ok(())
}
