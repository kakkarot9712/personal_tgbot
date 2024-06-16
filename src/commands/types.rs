use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum SimpleCommands {
    // #[command(description = "List all Menues.")]
    // ListMenues,
    #[command(description = "Check Availability.")]
    Ping,
    #[command(description = "Show Supported Commands")]
    Help,
    #[command(description = "Change Bot Mode")]
    ChangeMode,
}

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase")]
pub enum HiddenCommands {
    Start,
    Cancel,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are available for Expense Tracker Mode:"
)]
pub enum ExpenseTrackerCommands {
    #[command(description = "List Pending Dues")]
    ListAllDues,
    #[command(description = "Add New Person")]
    AddPerson,
    #[command(description = "Add New Transaction")]
    AddTransaction,
    #[command(description = "Settle Due")]
    SettleDue,
}
