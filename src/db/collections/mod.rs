use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub balance: u64,
}

impl Person {
    pub fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("persons")
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
