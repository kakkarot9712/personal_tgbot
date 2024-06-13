use std::{num::ParseFloatError, sync::Arc};

use mongodb::{bson::doc, Database};
use teloxide::{
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, Message},
    Bot,
};

use crate::{
    database::{schema::Person, traits::CollectionHelpers},
    dialogue::state::{DialogueWithState, State},
};

pub async fn handle_person_asked(
    q: CallbackQuery,
    bot: Bot,
    dialogue: DialogueWithState,
    db: Arc<Database>,
) -> ResponseResult<()> {
    let data = q.data.unwrap();
    if let Some(Message { id, chat, .. }) = q.message {
        let person = Person::find_by_id(&data, &db).await.unwrap().unwrap();
        let name = person.name.clone();
        let balance = person.balance.clone();
        dialogue
            .update(State::SDAmountAsked { person })
            .await
            .unwrap();
        bot.answer_callback_query(q.id).await.unwrap();
        bot.edit_message_text(
            chat.id,
            id,
            format!(
                "Selected Person: {}\n\n Current Due: {}\n\nEnter Amount To Settle \n(To Settle all due send 0)",
                name,
                (balance * 100.0).round() / 100.0
            ),
        )
        .await
        .unwrap();
    }
    Ok(())
}

pub async fn handle_amount_asked(
    bot: Bot,
    msg: Message,
    dialogue: DialogueWithState,
    person: Person,
    db: Arc<Database>,
) -> ResponseResult<()> {
    let data = msg.text().unwrap();
    let settle_balance: Result<f64, ParseFloatError> = data.parse();
    let Message { chat, .. } = msg;
    match settle_balance {
        Ok(b) => {
            if b > person.balance || b <= 0.0 {
                bot.send_message(
                    chat.id,
                    "Settlement Value cannot be more then Due itself or less then 0",
                )
                .await
                .unwrap();
            } else {
                let message = bot.send_message(chat.id, "Updating...").await.unwrap();
                let balance = if b == 0.0 {
                    b
                } else {
                    f64::trunc((person.balance - b) * 100.0) / 100.0
                };
                Person::find_by_id_and_update(
                    &person.id.unwrap().to_string(),
                    &db,
                    doc! { "$set": doc!{"balance": balance }},
                )
                .await
                .unwrap();
                dialogue.update(State::Idle).await.unwrap();
                bot.edit_message_text(chat.id, message.id, "Settlement Updated Successfully!")
                    .await
                    .unwrap();
            }
        }
        Err(_) => {
            bot.send_message(chat.id, "Please enter valid number")
                .await
                .unwrap();
        }
    }
    Ok(())
}
