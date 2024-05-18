use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum SimpleCommands {
    #[command(description = "List all Menues.")]
    ListMenues,
    #[command(description = "Check Availability.")]
    Ping,
    #[command(description = "Show Supported Commands")]
    Help,
}

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase")]
pub enum HiddenCommands {
    Start,
    Cancel,
}
