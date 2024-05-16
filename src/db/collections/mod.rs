use std::time;

use chrono::{DateTime, Utc};
use mongodb::{bson::oid::ObjectId, error::Error, Collection, Database};
use serde::{Deserialize, Serialize};

use super::{CollectionHandle, DBHandle};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub balance: f64,
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
    pub amount: f64,
    pub note: String,
    pub date: String,
}

impl Transaction {
    pub fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("transactions")
    }

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
