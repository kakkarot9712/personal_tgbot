use teloxide::{prelude::*, utils::command::BotCommands};

use crate::menus::modes::Modes;

use super::definitions::SimpleCommands;

pub async fn handle_commands(bot: Bot, msg: Message, cmd: SimpleCommands) -> ResponseResult<()> {
    match cmd {
        SimpleCommands::Ping => {
            bot.send_message(msg.chat.id, "PONG").await.unwrap();
        }
        SimpleCommands::Help => {
            bot.send_message(msg.chat.id, SimpleCommands::descriptions().to_string())
                .await
                .unwrap();
        }
        SimpleCommands::ListMenues => {
            let keyboard = Modes::make_keyboard();
            bot.send_message(msg.chat.id, "Available Features:")
                .reply_markup(keyboard)
                .await
                .unwrap();
        }
    };
    Ok(())
}
