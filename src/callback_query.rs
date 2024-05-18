use std::sync::Arc;

use mongodb::Database;
use teloxide::{requests::ResponseResult, types::CallbackQuery, Bot};

use crate::dialogue::{
    add_person_diag,
    add_transaction_diag::{self, split},
};

use traits::*;

use self::{expensetracker::add_transaction, modes::Modes};

pub mod expensetracker;
pub mod modes;
pub mod traits;

pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: add_person_diag::DialogueWithState,
    add_transaction_diag: add_transaction_diag::DialogueWithState,
    add_transaction_split_diag: split::DialogueWithState,
    db: Arc<Database>,
) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        if Modes::has_query(&data) {
            Modes::handle_callback(bot, q, data).await.unwrap();
        } else if expensetracker::layout::ButtonLayout::has_query(&data) {
            expensetracker::layout::ButtonLayout::handle_callback(bot, q, data, db, dialogue)
                .await
                .unwrap();
        } else if add_transaction::layout::ButtonLayout::has_query(&data) {
            add_transaction::layout::ButtonLayout::callback_query_handler(
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
