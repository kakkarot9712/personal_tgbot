use crate::{
    callback_query,
    commands::{self, types::*},
    dialogue::{
        add_person_diag,
        add_transaction_diag::{self, split},
    },
};
use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, di::DependencyMap, Handler},
    types::Update,
    RequestError,
};

// use dptree

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
        .branch(
            dptree::case![add_transaction_diag::State::Started]
                .endpoint(add_transaction_diag::handler::start),
        )
        .branch(
            dptree::case![add_transaction_diag::State::AmountAsked { amount }]
                .endpoint(add_transaction_diag::handler::handle_amount_asked),
        )
        .branch(
            dptree::case![split::State::Started]
                .endpoint(add_transaction_diag::split::handler::start),
        )
        .branch(
            dptree::case![split::State::AmountAsked { amount }]
                .endpoint(add_transaction_diag::split::handler::handle_amount_asked),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dialogue_handler)
        .branch(hidden_command_handler);

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            dptree::case![split::State::NoteAsked {
                amount,
                note,
                persons
            }]
            .endpoint(split::handler::handle_callback_query),
        )
        .branch(dptree::endpoint(callback_query::handle_callback));

    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<add_person_diag::State>, add_person_diag::State>()
        .enter_dialogue::<Update, InMemStorage<add_transaction_diag::State>, add_transaction_diag::State>()
        .enter_dialogue::<Update, InMemStorage<split::State>, split::State>(
        )
        .branch(message_handler)
        .branch(callback_query_handler)
}
