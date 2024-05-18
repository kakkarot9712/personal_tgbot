pub mod add_transaction;
pub mod layout;

use std::sync::Arc;

use super::traits::KeyboardGenerator;
use layout::ButtonLayout;

use mongodb::Database;
use teloxide::{
    prelude::*,
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, Message},
    Bot,
};

use crate::{
    database::{schema::Person, traits::CollectionHandle},
    dialogue::add_person_diag,
};

impl ButtonLayout {
    pub async fn handle_callback(
        bot: Bot,
        q: CallbackQuery,
        data: String,
        db: Arc<Database>,
        dialogue: add_person_diag::DialogueWithState,
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
                bot.edit_message_text(chat.id, id, "Select Split Mode:")
                    .await
                    .unwrap();
                bot.edit_message_reply_markup(chat.id, id)
                    .reply_markup(add_transaction::layout::ButtonLayout::make_keyboard())
                    .await
                    .unwrap();
            }
        }
        Ok(())
    }
}
