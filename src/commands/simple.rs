use dotenv_codegen::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{dialogue::mode, globals::Globals};

use super::types::{ExpenseTrackerCommands, SimpleCommands};

pub async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: SimpleCommands,
    chat_mode: mode::DialogueWithModeState,
    globals: Globals,
) -> ResponseResult<()> {
    match cmd {
        SimpleCommands::Ping => {
            bot.send_message(msg.chat.id, "PONG").await.unwrap();
        }
        SimpleCommands::Help => {
            let mut supported_commands = SimpleCommands::descriptions().to_string();
            if chat_mode.get().await.unwrap().unwrap().to_string()
                == mode::ModeState::ExpenseTracker.to_string()
            {
                supported_commands = format!(
                    "{}\n\nCurrent Mode: Expense Tracker Mode\n\n{}",
                    supported_commands,
                    ExpenseTrackerCommands::descriptions().to_string()
                );
            }
            bot.send_message(msg.chat.id, supported_commands)
                .await
                .unwrap();
        }
        SimpleCommands::ChangeMode => {
            let mode = chat_mode.get().await.unwrap().unwrap();
            match mode {
                mode::ModeState::ExpenseTracker => {
                    bot.send_message(msg.chat.id, "Currently, Only one mode is supported! which is currently active mode: Expense Tracker Mode.").await.unwrap();
                }
            }
        }
        SimpleCommands::Source => {
            bot.send_message(
            msg.chat.id,
            "Here is the source code of this bot.\nhttps://github.com/kakkarot9712/personal_tgbot",
        ).await.unwrap();
        }
        SimpleCommands::SwitchAccessMode => {
            let my_id = dotenv!("MYID");
            let sender = msg.from();
            if let Some(user) = sender {
                if my_id == user.id.to_string() {
                    let mut globals = globals.lock().await;
                    let current_mode = globals.get("allowAll");
                    if let Some(access_mode) = current_mode {
                        if access_mode == "true" {
                            globals.insert("allowAll".to_owned(), "false".to_owned());
                            bot.send_message(
                                msg.chat.id,
                                "Now mode specific features are restricted!",
                            )
                            .await
                            .unwrap();
                        } else {
                            bot.send_message(
                                msg.chat.id,
                                "Now Anyone can access mode specific features!",
                            )
                            .await
                            .unwrap();
                            globals.insert("allowAll".to_owned(), "true".to_owned());
                        }
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            "Now Anyone can access mode specific features!",
                        )
                        .await
                        .unwrap();
                        globals.insert("allowAll".to_owned(), "true".to_owned());
                    }
                } else {
                    bot.send_message(msg.chat.id, "Operation Forbidden!")
                        .await
                        .unwrap();
                }
            } else {
                bot.send_message(msg.chat.id, "Operation Forbidden!")
                    .await
                    .unwrap();
            }
        }
    };
    Ok(())
}
