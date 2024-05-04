use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum SimpleCommands {
    #[command(description = "Check Availability.")]
    Ping,
    #[command(description = "Show Supported Commands")]
    Help,
    #[command(description = "Get List Of All Person Dues")]
    ListDues,
    #[command(description = "Add New Person")]
    AddPerson,
    #[command(description = "Add New Transaction")]
    AddTransaction
}
