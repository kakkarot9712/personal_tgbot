use std::sync::Arc;

use mongodb::Database;
use teloxide::{requests::ResponseResult, types::CallbackQuery, Bot};

use crate::dialogue::{add_person_diag, add_transaction_diag::split, settle_due};

use traits::*;

use self::modes::Modes;

pub mod expensetracker;
pub mod modes;
pub mod traits;

pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: add_person_diag::DialogueWithState,
    add_transaction_split_diag: split::DialogueWithState,
    settle_due_diag: settle_due::DialogueWithState,
    db: Arc<Database>,
) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        if Modes::has_query(&data) {
            Modes::handle_callback(bot, q, data).await.unwrap();
        } else if expensetracker::layout::ButtonLayout::has_query(&data) {
            expensetracker::layout::ButtonLayout::handle_callback(
                bot,
                q,
                data,
                db,
                dialogue,
                add_transaction_split_diag,
                settle_due_diag,
            )
            .await
            .unwrap();
        } else {
        }
    }
    Ok(())
}
