use mongodb::{bson::Document, Collection, Database};
use std::future::Future;

pub trait DBHandle
where
    Self: Sized,
{
    fn get_collection_handle(db: &Database) -> Collection<Self>;
}

pub trait CollectionHelpers
where
    Self: DBHandle,
{
    fn get_all(db: &Database) -> impl Future<Output = Result<Vec<Self>, mongodb::error::Error>>;
    fn find_by_id(
        id: &String,
        db: &Database,
    ) -> impl Future<Output = Result<Option<Self>, mongodb::error::Error>>;

    fn find_by_id_and_update(
        id: &String,
        db: &Database,
        update: Document,
    ) -> impl Future<Output = Result<Option<Self>, mongodb::error::Error>>;
}
