use async_trait::async_trait;
use mongodb::Collection;
use mongodb::bson::doc;

use crate::config::mongo_config::MongoDbClient;
use crate::models::user_details::UserDetails;
use crate::error::{Result, self};

use super::RepositoryInterface;

const COLL_NAME: &str = "users";


#[derive(Debug)]
pub struct UserRepository;

#[async_trait]
impl RepositoryInterface<UserDetails, String> for UserRepository { 

    async fn save(user: UserDetails) -> Result<UserDetails> { 
        let collection = Self::collection();
        
        let insert_res = collection
            .insert_one(user, None)
            .await?;
        
        let filter = doc! {"_id": &insert_res.inserted_id};

        return collection
            .find_one(filter, None)
            .await?
            .ok_or(error::ServerError::NotFound)
    }

    async fn delete(_id: String) -> Result<Option<UserDetails>> { 

        todo!()
    }

    async fn partial_update(user: &UserDetails) -> Result<Option<UserDetails>> {
        todo!()
    }


    async fn get_one(_id: String) -> Result<Option<UserDetails>> {
        let collection = UserRepository::collection();
        let filter = doc! {"_id": &_id};
        
        return collection.find_one(filter, None)
            .await
            .map(|a| a)
            .map_err(|_| error::ServerError::NotFound);
    }

    fn collection() -> Collection<UserDetails> { 
        MongoDbClient::get_collection(COLL_NAME)
    }

    
}