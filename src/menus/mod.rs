use std::sync::Arc;

use mongodb::Database;
use teloxide::{requests::ResponseResult, types::CallbackQuery, Bot};

use crate::dialogue::{add_transaction::AddTransactionDialogue, MyDialogue};

use self::{expensetracker::ExpenseTrackerButtons, modes::Modes};

pub mod expensetracker;
pub mod modes;

pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    add_transaction_diag: AddTransactionDialogue,
    db: Arc<Database>,
) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        if Modes::has_query(&data) {
            Modes::handle_callback(bot, q, data).await.unwrap();
        } else if ExpenseTrackerButtons::has_query(&data) {
            ExpenseTrackerButtons::handle_callback(bot, q, data, db, dialogue, add_transaction_diag)
                .await
                .unwrap();
        }
    }
    Ok(())
}
