// use chrono::prelude::{DateTime, Utc};
use std::{num::ParseIntError, sync::Arc};

use dotenv_codegen::dotenv;
use mongodb::{bson::doc, Database};
use teloxide::{
    prelude::*,
    requests::{Requester, ResponseResult},
    types::Message,
    Bot,
};

use crate::{
    database::{
        schema::{Person, Transaction},
        traits::{CollectionHelpers, DBHandle},
    },
    dialogue::state::{DialogueWithState, State},
};

use super::insert_cancel_helper_text;

pub async fn start(bot: Bot, msg: Message, dialogue: DialogueWithState) -> ResponseResult<()> {
    let parsed_amt: Result<i64, ParseIntError> = msg.text().unwrap().parse();
    match parsed_amt {
        Ok(amt) => {
            bot.send_message(
                msg.chat.id,
                insert_cancel_helper_text("Describe Transaction.".to_owned()),
            )
            .await
            .unwrap();
            dialogue
                .update(State::TAmountAsked { amount: amt })
                .await
                .unwrap();
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "Enter Valid Amount")
                .await
                .unwrap();
        }
    };
    Ok(())
}

pub async fn handle_amount_asked(
    bot: Bot,
    msg: Message,
    dialogue: DialogueWithState,
    amount: i64,
    db: Arc<Database>,
) -> ResponseResult<()> {
    let note = msg.text().unwrap().to_owned();
    let keyboard = Person::make_keyboard(db, true).await;
    bot.send_message(
        msg.chat.id,
        insert_cancel_helper_text("Select The Persons:".to_owned()),
    )
    .reply_markup(keyboard)
    .await
    .unwrap();
    dialogue
        .update(State::TNoteAsked {
            amount,
            note,
            persons: None,
        })
        .await
        .unwrap();
    Ok(())
}

pub async fn handle_callback_query(
    bot: Bot,
    q: CallbackQuery,
    dialogue: DialogueWithState,
    db: Arc<Database>,
) -> ResponseResult<()> {
    bot.answer_callback_query(q.id).await.unwrap();
    if let Some(Message { id, chat, .. }) = q.message {
        let state: State = dialogue.get().await.unwrap().unwrap();
        if let State::TNoteAsked {
            amount,
            note,
            persons,
        } = state
        {
            let mut added_persons = persons.unwrap_or(Vec::new());
            let persons = Person::get_all(&db).await.unwrap();
            let msg = q.data.unwrap();
            let selected_person = persons.iter().find(|p| p.id.unwrap().to_string() == msg);
            if msg == "####done####" {
                bot.delete_message(chat.id, id).await.unwrap();
                let message = bot
                    .send_message(chat.id, "Adding Transaction...")
                    .await
                    .unwrap();
                let split_amount = amount as f64 / added_persons.len() as f64;
                let handle = Person::get_collection_handle(&db);
                let my_id = dotenv!("MYID");
                for p in added_persons.iter() {
                    let balance = p.balance as f64 + split_amount;
                    let balance = f64::trunc(balance * 100.0) / 100.0;
                    let filter = doc! { "_id": p.id.unwrap() };
                    let update = doc! { "$set": doc!{"balance": balance }};
                    handle
                        .find_one_and_update(filter, update, None)
                        .await
                        .unwrap();
                    if p.id.unwrap().to_string() == my_id {
                        Transaction::insert_one(split_amount, note.clone(), &db)
                            .await
                            .unwrap();
                    }
                }
                dialogue.update(State::Idle).await.unwrap();
                bot.edit_message_text(chat.id, message.id, "Success!")
                    .await
                    .unwrap();
            } else if added_persons
                .iter()
                .find(|p| p.id.unwrap().to_string() == msg)
                .is_some()
            {
                added_persons = added_persons
                    .into_iter()
                    .filter(|p| p.id.unwrap().to_string() != msg)
                    .collect();
                bot.send_message(
                    chat.id,
                    format!("Removed {}", selected_person.unwrap().name),
                )
                .await
                .unwrap();
            } else {
                bot.send_message(chat.id, format!("Added {}", selected_person.unwrap().name))
                    .await
                    .unwrap();
                added_persons.push(selected_person.unwrap().to_owned());
            }
            if msg != "####done####" {
                dialogue
                    .update(State::TNoteAsked {
                        amount,
                        note,
                        persons: Some(added_persons),
                    })
                    .await
                    .unwrap();
            }
        }
    }
    Ok(())
}
