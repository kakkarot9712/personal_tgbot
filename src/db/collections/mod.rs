use mongodb::{error::Error, Collection, Database};
use serde::{Deserialize, Serialize};

use super::{CollectionHandle, DBHandle};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub balance: u64,
}

impl DBHandle for Person {
    fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("persons")
    }
}

impl CollectionHandle for Person {
    async fn get_all(db: &Database) -> Result<Vec<Self>, Error> {
        let mut docs: Vec<Self> = Vec::new();
        let handle = Self::get_collection_handle(db);
        let mut col = handle.find(None, None).await?;
        while col.advance().await? {
            let doc = col.deserialize_current()?;
            docs.push(doc);
        }
        Ok(docs)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub amount: i64,
    pub note: String,
    pub date: String,
}

impl Transaction {
    pub fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("transactions")
    }
}
