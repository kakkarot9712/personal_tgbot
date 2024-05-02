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
    #[command(description = "Get List Of All Persons")]
    ListPersons,
    #[command(description = "Add New Person", parse_with = "split")]
    AddPerson,
}
