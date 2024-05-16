use std::{num::ParseFloatError, sync::Arc};

use mongodb::{
    // bson::{doc, oid::ObjectId},
    Database,
};
use teloxide::{
    requests::{Requester, ResponseResult},
    types::Message,
    Bot,
};

use crate::db::{
    collections::Transaction,
    // DBHandle,
};

use super::{AddTransactionDialogue, AddTransactionState};

pub async fn start(
    bot: Bot,
    msg: Message,
    add_transaction_diag: AddTransactionDialogue,
) -> ResponseResult<()> {
    let parsed_amt: Result<f64, ParseFloatError> = msg.text().unwrap().parse();
    match parsed_amt {
        Ok(amt) => {
            bot.send_message(msg.chat.id, "Describe Transaction.")
                .await
                .unwrap();
            add_transaction_diag
                .update(AddTransactionState::AmountAsked { amount: amt })
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
    add_transaction_diag: AddTransactionDialogue,
    amount: f64,
    db: Arc<Database>,
) -> ResponseResult<()> {
    let note = msg.text().unwrap().to_owned();
    Transaction::insert_one(amount, note, &db).await.unwrap();
    // let my_id = env!("MYID");
    // let handle = Person::get_collection_handle(&db);

    // TODO: Fix Me, Update my current Balance

    bot.send_message(msg.chat.id, "Inserted Successfully!")
        .await
        .unwrap();
    add_transaction_diag
        .update(AddTransactionState::Idle)
        .await
        .unwrap();
    Ok(())
}
