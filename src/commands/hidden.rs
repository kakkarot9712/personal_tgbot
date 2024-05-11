use teloxide::{macros::BotCommands, prelude::*, requests::ResponseResult, types::Message, Bot};

use crate::dialogue::{
    add_person_diag::AddPersonDialogueState,
    add_transaction::{
        split::{AddSplitTransactionDialogue, AddSplitTransactionState},
        AddTransactionDialogue, AddTransactionState,
    },
    MyDialogue,
};

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase")]
pub enum HiddenCommands {
    Start,
    Cancel,
}

impl HiddenCommands {
    pub async fn handle_commands(
        bot: Bot,
        msg: Message,
        cmd: HiddenCommands,
        add_person_diag: MyDialogue,
        add_transaction_diag: AddTransactionDialogue,
        add_transaction_split_diag: AddSplitTransactionDialogue,
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
                if add_person_state != AddPersonDialogueState::Idle {
                    add_person_diag
                        .update(AddPersonDialogueState::Idle)
                        .await
                        .unwrap();

                    bot.send_message(msg.chat.id, "Add Person Operation Canceled!")
                        .await
                        .unwrap();
                }
                if add_trasaction_diag_state != AddTransactionState::Idle {
                    add_transaction_diag
                        .update(AddTransactionState::Idle)
                        .await
                        .unwrap();

                    bot.send_message(msg.chat.id, "Add Transaction Operation Canceled!")
                        .await
                        .unwrap();
                }
                if add_transaction_split_diag_state != AddSplitTransactionState::Idle {
                    add_transaction_split_diag
                        .update(AddSplitTransactionState::Idle)
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
