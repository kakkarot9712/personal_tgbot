use std::sync::Arc;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection, Database,
};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::database::{
    schema::Person,
    traits::{CollectionHelpers, DBHandle},
};

impl DBHandle for Person {
    fn get_collection_handle(db: &Database) -> Collection<Self> {
        db.collection::<Self>("persons")
    }
}

impl CollectionHelpers for Person {
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

    async fn find_by_id(id: &String, db: &Database) -> Result<Option<Self>, mongodb::error::Error> {
        let handle = Self::get_collection_handle(&db);
        let id = ObjectId::parse_str(id).unwrap();
        let cursor = handle.find_one(doc! {"_id": id}, None).await?;
        Ok(cursor)
    }

    async fn find_by_id_and_update(
        id: &String,
        db: &Database,
        update: Document,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        let handle = Self::get_collection_handle(&db);
        let id = ObjectId::parse_str(id).unwrap();
        let person = handle
            .find_one_and_update(doc! {"_id": id}, update, None)
            .await
            .unwrap();
        Ok(person)
    }
}

impl Person {
    pub async fn make_keyboard(
        db: Arc<Database>,
        with_complete_selection_button: bool,
    ) -> InlineKeyboardMarkup {
        let persons = Self::get_all(&db).await.unwrap();
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        for chunk in persons.chunks(2) {
            let row = chunk
                .iter()
                .map(|p| InlineKeyboardButton::callback(p.name.clone(), p.id.unwrap().to_string()))
                .collect();

            keyboard.push(row);
        }
        if with_complete_selection_button {
            keyboard.push(vec![InlineKeyboardButton::callback(
                "Complete Selection",
                "####done####",
            )]);
        }
        InlineKeyboardMarkup::new(keyboard)
    }
}
