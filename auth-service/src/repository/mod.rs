use async_trait::async_trait;
use mongodb::Collection;
use crate::{error::Result};

mod account_repo;
mod user_repo;
mod link_repo;


/*

    ** Likely going to find another way to simplify this
    **

*/
#[async_trait]
pub trait RepositoryInterface<T, Id> { 
    async fn get_one(_id: Id) -> Result<Option<T>>;
    async fn save(entity: T) -> Result<T>;
    async fn delete(_id: Id) -> Result<Option<T>>;
    async fn partial_update(entity: &T) -> Result<Option<T>>;
    fn collection() -> Collection<T>;
}