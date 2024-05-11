// use chrono::prelude::{DateTime, Utc};
use std::{num::ParseIntError, sync::Arc};

use mongodb::Database;
use teloxide::{
    prelude::*,
    requests::{Requester, ResponseResult},
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::db::{collections::Person, CollectionHandle};

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
            .map(|p| InlineKeyboardButton::callback(p.name.clone(), p.name.clone()))
            .collect();

        keyboard.push(row);
    }

    bot.send_message(msg.chat.id, "Select The Persons:")
        .reply_markup(InlineKeyboardMarkup::new(keyboard))
        .await
        .unwrap();
    add_transaction_diag
        .update(AddSplitTransactionState::NoteAsked { amount, note })
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
    let state: AddSplitTransactionState = add_transaction_diag.get().await.unwrap().unwrap();
    println!("{:?}", state);
    // TODO: Add Way to Allow Multiple Person Selection
    Ok(())
}
