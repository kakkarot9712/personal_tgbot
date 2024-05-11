use std::fmt;

use serde::{Deserialize, Serialize};
use teloxide::{
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::dialogue::add_transaction::{
    split::{AddSplitTransactionDialogue, AddSplitTransactionState},
    AddTransactionDialogue, AddTransactionState,
};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum AddTransactionType {
    #[serde(rename = "Single User")]
    Solo,
    #[serde(rename = "Multiple User")]
    SplitEq,
}

impl fmt::Display for AddTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl AddTransactionType {
    fn get_available_options() -> Vec<String> {
        vec![
            AddTransactionType::Solo.to_string(),
            AddTransactionType::SplitEq.to_string(),
        ]
    }

    pub fn has_query(q: &String) -> bool {
        let available_modes = Self::get_available_options();
        available_modes.contains(q)
    }

    pub fn make_keyboard() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        let available_commands = Self::get_available_options();
        for commands in available_commands.chunks(3) {
            let row = commands
                .iter()
                .map(|cmd| InlineKeyboardButton::callback(cmd.to_owned(), cmd.to_owned()))
                .collect();

            keyboard.push(row);
        }
        InlineKeyboardMarkup::new(keyboard)
    }

    pub async fn callback_query_handler(
        bot: Bot,
        data: String,
        q: CallbackQuery,
        add_transaction_diag: AddTransactionDialogue,
        add_split_transaction_diag: AddSplitTransactionDialogue,
    ) -> ResponseResult<()> {
        if let Some(Message { id, chat, .. }) = q.message {
            if data == AddTransactionType::Solo.to_string() {
                add_transaction_diag
                    .update(AddTransactionState::Started)
                    .await
                    .unwrap();
            } else if data == AddTransactionType::SplitEq.to_string() {
                add_split_transaction_diag
                    .update(AddSplitTransactionState::Started)
                    .await
                    .unwrap();
            }
            bot.edit_message_text(chat.id, id, "Okay! Enter the Amount.")
                .await
                .unwrap();
        }
        bot.answer_callback_query(q.id).await.unwrap();
        Ok(())
    }
}
