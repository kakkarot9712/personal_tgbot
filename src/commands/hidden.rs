use teloxide::{prelude::*, requests::ResponseResult, types::Message, Bot};

use crate::dialogue::state::{DialogueWithState, State};

use super::types::HiddenCommands;

impl HiddenCommands {
    pub async fn handle_commands(
        bot: Bot,
        msg: Message,
        cmd: HiddenCommands,
        dialogue: DialogueWithState,
    ) -> ResponseResult<()> {
        match cmd {
            Self::Start => {
                bot.send_message(msg.chat.id, "Hello! This is Telegram Bot made by @Kakkarto9712. To know more about this bot send /help message.").await.unwrap();
            }

            Self::Cancel => {
                let current_state = dialogue.get().await.unwrap().unwrap();
                if current_state != State::Idle {
                    dialogue.update(State::Idle).await.unwrap();

                    bot.send_message(msg.chat.id, "Operation Canceled!")
                        .await
                        .unwrap();
                }
            }
        }
        Ok(())
    }
}
