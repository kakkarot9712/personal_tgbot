use std::fmt;

use serde::{Deserialize, Serialize};
use teloxide::{
    payloads::SendMessageSetters,
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::menus::expensetracker::ExpenseTrackerButtons;

#[derive(Debug, Deserialize, Serialize)]
pub enum Modes {
    #[serde(rename = "Manage Expenses")]
    ExpenseTracker,
    #[serde(rename = "Placeholder")]
    TestMode,
}

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Modes {
    fn get_available_options() -> Vec<String> {
        vec![
            Modes::ExpenseTracker.to_string(),
            Modes::TestMode.to_string(),
        ]
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

    pub fn has_query(q: &String) -> bool {
        let available_modes = Self::get_available_options();
        available_modes.contains(q)
    }

    pub async fn handle_callback(bot: Bot, q: CallbackQuery, data: String) -> ResponseResult<()> {
        bot.answer_callback_query(q.id).await?;
        if let Some(Message { id, chat, .. }) = q.message {
            // bot.edit_message_text(chat.id, id, format!("Showing Options for {}", data))
            //     .await
            //     .unwrap();
            if data == Modes::ExpenseTracker.to_string() {
                bot.send_message(chat.id,"Options For Expense Tracker are:")
                    .reply_markup(ExpenseTrackerButtons::make_keyboard())
                    .await
                    .unwrap();
            
            } else if data == Modes::TestMode.to_string() {
            }
            bot.delete_message(chat.id, id).await.unwrap();
        }
        Ok(())
    }
}
