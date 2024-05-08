use std::{fmt, sync::Arc};

use mongodb::Database;
use serde::{Deserialize, Serialize};
use teloxide::{
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::{
    db::collections::Person,
    dialogue::{
        add_person_diag::AddPersonDialogueState,
        add_transaction::{AddTransactionDialogue, AddTransactionState},
        MyDialogue,
    },
};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ExpenseTrackerButtons {
    #[serde(rename = "List all Dues")]
    ListDues,
    #[serde(rename = "Add a Peson")]
    AddPerson,
    #[serde(rename = "Add a Transaction")]
    AddTransaction,
}

impl fmt::Display for ExpenseTrackerButtons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ExpenseTrackerButtons {
    fn get_available_options() -> Vec<String> {
        vec![
            ExpenseTrackerButtons::ListDues.to_string(),
            ExpenseTrackerButtons::AddPerson.to_string(),
            ExpenseTrackerButtons::AddTransaction.to_string(),
        ]
    }

    pub fn has_query(q: &String) -> bool {
        let available_modes = Self::get_available_options();
        available_modes.contains(q)
    }

    pub fn make_keyboard() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        let available_commands = Self::get_available_options();

        for commands in available_commands.chunks(2) {
            let row = commands
                .iter()
                .map(|cmd| InlineKeyboardButton::callback(cmd.to_owned(), cmd.to_owned()))
                .collect();

            keyboard.push(row);
        }
        InlineKeyboardMarkup::new(keyboard)
    }

    pub async fn handle_callback(
        bot: Bot,
        q: CallbackQuery,
        data: String,
        db: Arc<Database>,
        dialogue: MyDialogue,
        add_transaction_diag: AddTransactionDialogue,
    ) -> ResponseResult<()> {
        if let Some(Message { id, chat, .. }) = q.message {
            if data == ExpenseTrackerButtons::ListDues.to_string() {
                let mut formatted_msg = String::from("");
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
                            bot.send_message(chat.id, "DB Operation Failed!")
                                .await
                                .unwrap();
                            break;
                        }
                    };
                    let person = cursor.deserialize_current().unwrap();
                    formatted_msg.push_str(&format!("{} :- {}\n", person.name, person.balance));
                }
                bot.answer_callback_query(q.id).await?;
                bot.edit_message_text(chat.id, id, "List of Dues by Persons:")
                    .await
                    .unwrap();
                if formatted_msg == "" {
                    bot.send_message(
                        chat.id,
                        "No Users found! Please Add New Person to get Started.",
                    )
                    .await
                    .unwrap();
                } else {
                    bot.send_message(chat.id, formatted_msg).await.unwrap();
                }
            } else if data == ExpenseTrackerButtons::AddPerson.to_string() {
                dialogue
                    .update(AddPersonDialogueState::ReceiveName)
                    .await
                    .unwrap();
                bot.send_message(chat.id, format!("Okay! What is the Full Name of User?"))
                    .await
                    .unwrap();
                bot.delete_message(chat.id, id).await.unwrap();
            } else if data == ExpenseTrackerButtons::AddTransaction.to_string() {
                add_transaction_diag
                    .update(AddTransactionState::Started)
                    .await
                    .unwrap();
                bot.send_message(chat.id, "Okay! Enter the Amount.")
                    .await
                    .unwrap();
                bot.delete_message(chat.id, id).await.unwrap();
            }
        }
        Ok(())
    }
}
