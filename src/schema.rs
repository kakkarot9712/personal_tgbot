use crate::{
    callback_query,
    commands::{self, types::*},
    dialogue::{add_person_diag, add_transaction_diag::split, settle_due},
};
use dotenv_codegen::dotenv;
use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, di::DependencyMap, Handler},
    prelude::*,
    requests::ResponseResult,
    types::{Message, Update},
    Bot, RequestError,
};

fn only_me(msg: Message) -> bool {
    let my_id = dotenv!("MYID");
    let sender = msg.from();
    match sender {
        Some(u) => u.id.to_string() == my_id.to_string(),
        None => false,
    }
}

async fn not_allowed(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Only owner can use this bot as of now. If you want to use bot, kindly goto Github profile and follow deployment steps").await.unwrap();
    Ok(())
}

async fn cb_not_allowed(cb: CallbackQuery, bot: Bot) -> ResponseResult<()> {
    if let Some(msg) = cb.message {
        not_allowed(bot, msg).await.unwrap();
    }
    Ok(())
}

fn cb_only_me(cb: CallbackQuery) -> bool {
    if let Some(msg) = cb.message {
        let is_me = only_me(msg);
        return is_me;
    }
    false
}

pub fn schema() -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    let command_handler = teloxide::filter_command::<SimpleCommands, _>().branch(
        dptree::case![add_person_diag::State::Idle]
            .filter_command::<SimpleCommands>()
            .endpoint(commands::simple::handle_commands),
    );

    let hidden_command_handler = dptree::entry()
        .filter_command::<HiddenCommands>()
        .endpoint(HiddenCommands::handle_commands);

    let dialogue_handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<HiddenCommands>()
                .endpoint(HiddenCommands::handle_commands),
        )
        .branch(
            dptree::case![add_person_diag::State::ReceiveName]
                .endpoint(add_person_diag::handler::handle_name),
        )
        .branch(
            dptree::case![add_person_diag::State::ReceiveBalance { full_name }]
                .endpoint(add_person_diag::handler::handle_due),
        )
        .branch(dptree::case![split::State::Started].endpoint(split::handler::start))
        .branch(
            dptree::case![split::State::AmountAsked { amount }]
                .endpoint(split::handler::handle_amount_asked),
        )
        .branch(
            dptree::case![settle_due::State::AmountAsked { person }]
                .endpoint(settle_due::handler::handle_amount_asked),
        );

    let message_handler = Update::filter_message()
        .branch(
            dptree::filter(only_me)
                .branch(command_handler)
                .branch(dialogue_handler)
                .branch(hidden_command_handler),
        )
        .endpoint(not_allowed);

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            dptree::filter(cb_only_me)
                .branch(
                    dptree::case![split::State::NoteAsked {
                        amount,
                        note,
                        persons
                    }]
                    .endpoint(split::handler::handle_callback_query),
                )
                .branch(
                    dptree::case![settle_due::State::PersonAsked]
                        .endpoint(settle_due::handler::handle_person_asked),
                )
                .branch(dptree::endpoint(callback_query::handle_callback)),
        )
        .endpoint(cb_not_allowed);

    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<add_person_diag::State>, add_person_diag::State>()
        .enter_dialogue::<Update, InMemStorage<split::State>, split::State>()
        .enter_dialogue::<Update, InMemStorage<settle_due::State>, settle_due::State>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
