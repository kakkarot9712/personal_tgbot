use teloxide::{prelude::*, utils::command::BotCommands};

use crate::dialogue::mode;

use super::types::{ExpenseTrackerCommands, SimpleCommands};

pub async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: SimpleCommands,
    chat_mode: mode::DialogueWithModeState,
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
    };
    Ok(())
}
