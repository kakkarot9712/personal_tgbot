use std::sync::Arc;

use mongodb::Database;
use teloxide::{requests::ResponseResult, types::CallbackQuery, Bot};

use crate::dialogue::{
    add_transaction::{split::AddSplitTransactionDialogue, AddTransactionDialogue},
    MyDialogue,
};

use self::{
    expensetracker::{add_transaction::AddTransactionType, ExpenseTrackerButtons},
    modes::Modes,
};

pub mod expensetracker;
pub mod modes;

pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    add_transaction_diag: AddTransactionDialogue,
    add_transaction_split_diag: AddSplitTransactionDialogue,
    db: Arc<Database>,
) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        if Modes::has_query(&data) {
            Modes::handle_callback(bot, q, data).await.unwrap();
        } else if ExpenseTrackerButtons::has_query(&data) {
            ExpenseTrackerButtons::handle_callback(bot, q, data, db, dialogue)
                .await
                .unwrap();
        } else if AddTransactionType::has_query(&data) {
            AddTransactionType::callback_query_handler(
                bot,
                data,
                q,
                add_transaction_diag,
                add_transaction_split_diag,
            )
            .await
            .unwrap();
        } else {
        }
    }
    Ok(())
}
