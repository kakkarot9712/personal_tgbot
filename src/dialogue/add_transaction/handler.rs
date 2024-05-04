use std::{num::ParseIntError, sync::Arc, time};
use chrono::prelude::{DateTime, Utc};

use mongodb::Database;
use teloxide::{requests::{Requester, ResponseResult}, types::Message, Bot};

use crate::db::collections::Transaction;

use super::{AddTransactionDialogue, AddTransactionState};

pub async fn start(bot: Bot, msg: Message, add_transaction_diag: AddTransactionDialogue) -> ResponseResult<()> {
    let parsed_amt:Result<i64, ParseIntError> = msg.text().unwrap().parse();
    match parsed_amt {
        Ok(amt) => {
            bot.send_message(msg.chat.id, "Describe Transaction.").await.unwrap();
            add_transaction_diag.update(AddTransactionState::AmountAsked { amount: amt }).await.unwrap();
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "Enter Valid Amount").await.unwrap();
        }
    };
    Ok(())
}

pub async fn handle_amount_asked(bot: Bot, msg: Message, add_transaction_diag: AddTransactionDialogue, amount: i64,db: Arc<Database>) -> ResponseResult<()> {
    let note = msg.text().unwrap().to_owned();
    let date = time::SystemTime::now();
    let iso_date: DateTime<Utc> = date.into();
    let col = Transaction::get_collection_handle(&db);
    col.insert_one(Transaction {amount, note, date: iso_date.to_rfc3339()}, None).await.unwrap();
    bot.send_message(msg.chat.id, "Inserted Successfully!").await.unwrap();
    add_transaction_diag.update(AddTransactionState::Idle).await.unwrap();
    Ok(())
}