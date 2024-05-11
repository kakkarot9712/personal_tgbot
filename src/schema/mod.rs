use crate::{
    commands::{self, definitions::SimpleCommands, hidden::HiddenCommands},
    dialogue::{
        add_person_diag::{handler, AddPersonDialogueState},
        add_transaction::{
            self,
            split::{self, AddSplitTransactionState},
            AddTransactionState,
        },
    },
    menus,
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
        dptree::case![AddPersonDialogueState::Idle]
            .filter_command::<SimpleCommands>()
            .endpoint(commands::handlers::handle_commands),
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
        .branch(dptree::case![AddPersonDialogueState::ReceiveName].endpoint(handler::handle_name))
        .branch(
            dptree::case![AddPersonDialogueState::ReceiveBalance { full_name }]
                .endpoint(handler::handle_due),
        )
        .branch(
            dptree::case![AddTransactionState::Started].endpoint(add_transaction::handler::start),
        )
        .branch(
            dptree::case![AddTransactionState::AmountAsked { amount }]
                .endpoint(add_transaction::handler::handle_amount_asked),
        )
        .branch(
            dptree::case![AddSplitTransactionState::Started]
                .endpoint(add_transaction::split::handler::start),
        )
        .branch(
            dptree::case![AddSplitTransactionState::AmountAsked { amount }]
                .endpoint(add_transaction::split::handler::handle_amount_asked),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dialogue_handler)
        .branch(hidden_command_handler);

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            dptree::case![AddSplitTransactionState::NoteAsked { amount, note }]
                .endpoint(split::handler::handle_callback_query),
        )
        .branch(dptree::endpoint(menus::handle_callback));

    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<AddPersonDialogueState>, AddPersonDialogueState>()
        .enter_dialogue::<Update, InMemStorage<AddTransactionState>, AddTransactionState>()
        .enter_dialogue::<Update, InMemStorage<AddSplitTransactionState>, AddSplitTransactionState>(
        )
        .branch(message_handler)
        .branch(callback_query_handler)
}
