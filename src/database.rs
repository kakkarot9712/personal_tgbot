pub mod collections;
pub mod schema;
pub mod traits;

use dotenv_codegen::dotenv;
use mongodb::{error::Error, options::ClientOptions, Client, Database};

pub async fn initialize_db() -> Result<Database, Error> {
    let client_options = ClientOptions::parse(format!(
        "mongodb+srv://{}:{}@{}",
        dotenv!("DB_USERNAME"),
        dotenv!("DB_PASSWORD"),
        dotenv!("DB_ENDPOINT")
    ))
    .await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("tgbot");
    Ok(db)
}
