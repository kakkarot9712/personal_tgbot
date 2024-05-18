use std::time;

use chrono::{DateTime, Utc};
use mongodb::{Collection, Database};

use crate::database::{schema::Transaction, traits::DBHandle};

impl DBHandle for Transaction {
    fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("transactions")
    }
}

impl Transaction {
    pub async fn insert_one(
        amount: f64,
        note: String,
        db: &Database,
    ) -> Result<(), mongodb::error::Error> {
        let handle = Self::get_collection_handle(db);
        let date = time::SystemTime::now();
        let iso_date: DateTime<Utc> = date.into();
        handle
            .insert_one(
                Transaction {
                    id: None,
                    amount,
                    date: iso_date.to_rfc3339(),
                    note,
                },
                None,
            )
            .await?;
        Ok(())
    }
}
