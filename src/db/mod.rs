use std::future::Future;

use dotenv_codegen::dotenv;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};
// use std::{fs::OpenOptions, io::{self, Read}};

pub mod collections;

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

pub trait DBHandle
where
    Self: Sized,
{
    fn get_collection_handle(db: &Database) -> Collection<Self>;
}

pub trait CollectionHandle
where
    Self: DBHandle,
{
    fn get_all(db: &Database) -> impl Future<Output = Result<Vec<Self>, Error>>;
}
