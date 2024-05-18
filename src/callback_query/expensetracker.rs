pub mod layout;

use layout::ButtonLayout;
use std::sync::Arc;

use mongodb::Database;
use teloxide::{
    prelude::*,
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, Message},
    Bot,
};

use crate::{
    database::{schema::Person, traits::CollectionHelpers},
    dialogue::{add_person_diag, add_transaction_diag::split, settle_due},
};

impl ButtonLayout {
    pub async fn handle_callback(
        bot: Bot,
        q: CallbackQuery,
        data: String,
        db: Arc<Database>,
        dialogue: add_person_diag::DialogueWithState,
        add_split_transaction_diag: split::DialogueWithState,
        settle_due_diag: settle_due::DialogueWithState,
    ) -> ResponseResult<()> {
        if let Some(Message { id, chat, .. }) = q.message {
            if data == ButtonLayout::ListDues.to_string() {
                let mut formatted_msg = String::from("");
                let docs = Person::get_all(&db).await.unwrap();
                for person in docs.iter() {
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
            } else if data == ButtonLayout::AddPerson.to_string() {
                dialogue
                    .update(add_person_diag::State::ReceiveName)
                    .await
                    .unwrap();
                bot.edit_message_text(chat.id, id, format!("Okay! What is the Full Name of User?"))
                    .await
                    .unwrap();
            } else if data == ButtonLayout::AddTransaction.to_string() {
                // bot.edit_message_text(chat.id, id, "Select Split Mode:")
                //     .await
                //     .unwrap();
                // bot.edit_message_reply_markup(chat.id, id)
                //     .reply_markup(add_transaction::layout::ButtonLayout::make_keyboard())
                //     .await
                //     .unwrap();
                add_split_transaction_diag
                    .update(split::State::Started)
                    .await
                    .unwrap();
                bot.edit_message_text(chat.id, id, "Okay! Enter the Amount.")
                    .await
                    .unwrap();
            } else if data == ButtonLayout::SettleDues.to_string() {
                let keyboard = Person::make_keyboard(db, false).await;
                settle_due_diag
                    .update(settle_due::State::PersonAsked)
                    .await
                    .unwrap();
                bot.edit_message_text(chat.id, id, "Okay Select The Person:")
                    .reply_markup(keyboard)
                    .await
                    .unwrap();
            }
        }
        Ok(())
    }
}
