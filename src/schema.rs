use crate::{
    auth::only_me,
    commands::{self, types::*},
    dialogue::{add_person_diag, add_transaction_diag, mode::ModeState, settle_due, state::State},
};
use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, di::DependencyMap, Handler},
    types::Update,
    RequestError,
};

pub fn schema() -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    let command_handler = dptree::entry().branch(
        dptree::case![State::Idle]
            .branch(
                dptree::entry()
                    .filter_command::<SimpleCommands>()
                    .endpoint(commands::simple::handle_commands),
            )
            .filter_async(only_me)
            .branch(
                dptree::case![ModeState::ExpenseTracker]
                    .filter_command::<ExpenseTrackerCommands>()
                    .endpoint(commands::expensetrackercommands::handle_commands),
            ),
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
        .branch(dptree::case![State::PReceiveName].endpoint(add_person_diag::handle_name))
        .branch(
            dptree::case![State::PReceiveBalance { full_name }]
                .endpoint(add_person_diag::handle_due),
        )
        .branch(dptree::case![State::TStarted].endpoint(add_transaction_diag::start))
        .branch(
            dptree::case![State::TAmountAsked { amount }]
                .endpoint(add_transaction_diag::handle_amount_asked),
        )
        .branch(
            dptree::case![State::SDAmountAsked { person }]
                .endpoint(settle_due::handle_amount_asked),
        );

    let message_handler = Update::filter_message()
        .branch(dialogue_handler)
        .branch(hidden_command_handler)
        .branch(command_handler);

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            dptree::case![State::TNoteAsked {
                amount,
                note,
                persons
            }]
            .endpoint(add_transaction_diag::handle_callback_query),
        )
        .branch(dptree::case![State::SDPersonAsked].endpoint(settle_due::handle_person_asked));

    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .enter_dialogue::<Update, InMemStorage<ModeState>, ModeState>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
