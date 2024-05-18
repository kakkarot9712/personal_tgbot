use mongodb::{Collection, Database};

use crate::database::{schema::Person, traits::{CollectionHandle, DBHandle}};

impl DBHandle for Person {
    fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("persons")
    }
}

impl CollectionHandle for Person {
    async fn get_all(db: &Database) -> Result<Vec<Self>, mongodb::error::Error> {
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
