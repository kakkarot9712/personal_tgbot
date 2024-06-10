use crate::database::{schema::User, traits::DBHandle};


impl DBHandle for User {
    fn get_collection_handle(db: &mongodb::Database) -> mongodb::Collection<Self> {
        db.collection("users")
    }
}
