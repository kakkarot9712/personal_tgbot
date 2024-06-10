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
    dialogue::state::{DialogueWithState, State},
};

impl ButtonLayout {
    pub async fn handle_callback(
        bot: Bot,
        q: CallbackQuery,
        data: String,
        db: Arc<Database>,
        dialogue: DialogueWithState,
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
                    .update(State::PReceiveName)
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
                dialogue.update(State::TStarted).await.unwrap();
                bot.edit_message_text(chat.id, id, "Okay! Enter the Amount.")
                    .await
                    .unwrap();
            } else if data == ButtonLayout::SettleDues.to_string() {
                let keyboard = Person::make_keyboard(db, false).await;
                dialogue.update(State::SDPersonAsked).await.unwrap();
                bot.edit_message_text(chat.id, id, "Okay Select The Person:")
                    .reply_markup(keyboard)
                    .await
                    .unwrap();
            }
        }
        Ok(())
    }
}
