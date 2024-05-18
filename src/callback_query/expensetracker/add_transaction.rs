pub mod layout;

use teloxide::{
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, Message},
    Bot,
};

use crate::dialogue::add_transaction_diag::{self, split};

use self::layout::ButtonLayout;

impl ButtonLayout {
    pub async fn callback_query_handler(
        bot: Bot,
        data: String,
        q: CallbackQuery,
        add_transaction_diag: add_transaction_diag::DialogueWithState,
        add_split_transaction_diag: split::DialogueWithState,
    ) -> ResponseResult<()> {
        if let Some(Message { id, chat, .. }) = q.message {
            if data == ButtonLayout::Solo.to_string() {
                add_transaction_diag
                    .update(add_transaction_diag::State::Started)
                    .await
                    .unwrap();
            } else if data == ButtonLayout::SplitEq.to_string() {
                add_split_transaction_diag
                    .update(split::State::Started)
                    .await
                    .unwrap();
            }
            bot.edit_message_text(chat.id, id, "Okay! Enter the Amount.")
                .await
                .unwrap();
        }
        bot.answer_callback_query(q.id).await.unwrap();
        Ok(())
    }
}
