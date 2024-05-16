// use chrono::prelude::{DateTime, Utc};
use std::{num::ParseIntError, sync::Arc};

use mongodb::{bson::doc, Database};
use teloxide::{
    prelude::*,
    requests::{Requester, ResponseResult},
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::db::{collections::Person, CollectionHandle, DBHandle};

use super::{AddSplitTransactionDialogue, AddSplitTransactionState};

pub async fn start(
    bot: Bot,
    msg: Message,
    add_transaction_diag: AddSplitTransactionDialogue,
) -> ResponseResult<()> {
    let parsed_amt: Result<i64, ParseIntError> = msg.text().unwrap().parse();
    match parsed_amt {
        Ok(amt) => {
            bot.send_message(msg.chat.id, "Describe Transaction.")
                .await
                .unwrap();
            add_transaction_diag
                .update(AddSplitTransactionState::AmountAsked { amount: amt })
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
    add_transaction_diag: AddSplitTransactionDialogue,
    amount: i64,
    db: Arc<Database>,
) -> ResponseResult<()> {
    let note = msg.text().unwrap().to_owned();
    let persons = Person::get_all(&db).await.unwrap();
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for chunk in persons.chunks(2) {
        let row = chunk
            .iter()
            .map(|p| InlineKeyboardButton::callback(p.name.clone(), p.id.unwrap().to_string()))
            .collect();

        keyboard.push(row);
    }
    keyboard.push(vec![InlineKeyboardButton::callback(
        "Complete Selection",
        "####done####",
    )]);

    bot.send_message(msg.chat.id, "Select The Persons:")
        .reply_markup(InlineKeyboardMarkup::new(keyboard))
        .await
        .unwrap();
    add_transaction_diag
        .update(AddSplitTransactionState::NoteAsked {
            amount,
            note,
            persons: None,
        })
        .await
        .unwrap();
    Ok(())
}

// pub async fn handle_note_asked(
//     bot: Bot,
//     msg: Message,
//     add_transaction_diag: AddSplitTransactionDialogue,
//     amount: i64,
//     note: String,
//     db: Arc<Database>,
// ) -> ResponseResult<()> {
//     // Do Some Calculations
//     let date = time::SystemTime::now();
//     let iso_date: DateTime<Utc> = date.into();
//     let col = Transaction::get_collection_handle(&db);
//     col.insert_one(
//         Transaction {
//             amount,
//             note,
//             date: iso_date.to_rfc3339(),
//         },
//         None,
//     )
//     .await
//     .unwrap();
//     bot.send_message(msg.chat.id, "Inserted Successfully!")
//         .await
//         .unwrap();
//     add_transaction_diag
//         .update(AddSplitTransactionState::Idle)
//         .await
//         .unwrap();
//     Ok(())
// }

pub async fn handle_callback_query(
    bot: Bot,
    q: CallbackQuery,
    add_transaction_diag: AddSplitTransactionDialogue,
    db: Arc<Database>,
) -> ResponseResult<()> {
    bot.answer_callback_query(q.id).await.unwrap();
    if let Some(Message { id, chat, .. }) = q.message {
        let state: AddSplitTransactionState = add_transaction_diag.get().await.unwrap().unwrap();
        if let AddSplitTransactionState::NoteAsked {
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
                for p in added_persons.iter() {
                    let filter = doc! { "_id": p.id.unwrap() };
                    let update = doc! { "$set": doc!{"balance": p.balance as f64 + split_amount }};
                    handle
                        .find_one_and_update(filter, update, None)
                        .await
                        .unwrap();
                }
                bot.edit_message_text(chat.id, message.id, "Success!")
                    .await
                    .unwrap();
                add_transaction_diag
                    .update(AddSplitTransactionState::Idle)
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

            add_transaction_diag
                .update(AddSplitTransactionState::NoteAsked {
                    amount,
                    note,
                    persons: Some(added_persons),
                })
                .await
                .unwrap();
        }
    }
    Ok(())
}
