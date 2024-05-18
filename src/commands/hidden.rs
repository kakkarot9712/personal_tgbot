use teloxide::{prelude::*, requests::ResponseResult, types::Message, Bot};

use crate::dialogue::{
    add_person_diag,
    add_transaction_diag::{
        split,
        self
    },
};

use super::types::HiddenCommands;

impl HiddenCommands {
    pub async fn handle_commands(
        bot: Bot,
        msg: Message,
        cmd: HiddenCommands,
        add_person_diag: add_person_diag::DialogueWithState,
        add_transaction_diag: add_transaction_diag::DialogueWithState,
        add_transaction_split_diag: split::DialogueWithState,
    ) -> ResponseResult<()> {
        match cmd {
            Self::Start => {
                bot.send_message(msg.chat.id, "Hello! This is Telegram Bot made by @Kakkarto9712. Currently WIP. To know more about this bot send /help message.").await.unwrap();
            }

            Self::Cancel => {
                let add_person_state = add_person_diag.get().await.unwrap().unwrap();
                let add_trasaction_diag_state = add_transaction_diag.get().await.unwrap().unwrap();
                let add_transaction_split_diag_state =
                    add_transaction_split_diag.get().await.unwrap().unwrap();
                if add_person_state != add_person_diag::State::Idle {
                    add_person_diag
                        .update(add_person_diag::State::Idle)
                        .await
                        .unwrap();

                    bot.send_message(msg.chat.id, "Add Person Operation Canceled!")
                        .await
                        .unwrap();
                }
                if add_trasaction_diag_state != add_transaction_diag::State::Idle {
                    add_transaction_diag
                        .update(add_transaction_diag::State::Idle)
                        .await
                        .unwrap();

                    bot.send_message(msg.chat.id, "Add Transaction Operation Canceled!")
                        .await
                        .unwrap();
                }
                if add_transaction_split_diag_state != split::State::Idle {
                    add_transaction_split_diag
                        .update(split::State::Idle)
                        .await
                        .unwrap();

                    bot.send_message(msg.chat.id, "Add Transaction Operation Canceled!")
                        .await
                        .unwrap();
                }
            }
        }
        Ok(())
    }
}
