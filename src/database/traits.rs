use std::future::Future;
use mongodb::{Collection, Database};

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
    fn get_all(db: &Database) -> impl Future<Output = Result<Vec<Self>, mongodb::error::Error>>;
}
