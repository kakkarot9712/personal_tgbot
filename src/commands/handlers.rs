use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    db::{pull_persons_data, Person},
    dialogue::definitions::{AddPersonDialogueState, MyDialogue},
};

use super::definitions::SimpleCommands;

pub async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: SimpleCommands,
    dialogue: MyDialogue,
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
        SimpleCommands::ListPersons => {
            let mut formatted_msg = String::new();
            let persons: Vec<Person> = pull_persons_data().expect("DB ERROR! Aborting...");
            for (i, p) in persons.iter().enumerate() {
                formatted_msg.push_str(&format!("{}. {}\n", i + 1, p.name));
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
    };
    Ok(())
}
