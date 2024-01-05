use async_trait::async_trait;
use mongodb::Collection;
use mongodb::bson::doc;

use crate::config::mongo_config::MongoDbClient;
use crate::models::user_details::UserDetails;
use crate::error::{Result, self};

const COLL_NAME: &str = "users";

#[async_trait]
pub trait UserInterface { 
    async fn get_one(_id: &str) -> Result<Option<UserDetails>>;
    async fn save(user: UserDetails) -> Result<UserDetails>;
    async fn delete(_id: &str);
    async fn partial_update(user: UserDetails) -> Result<Option<UserDetails>>;
    fn collection() -> Collection<UserDetails>;
}

#[derive(Debug)]
pub struct UserRepository;


#[async_trait]
impl UserInterface for UserRepository { 


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

    async fn delete(_id: &str) { 
        
    }

    async fn partial_update(user: UserDetails) -> Result<Option<UserDetails>> {
        todo!()
    }


    async fn get_one(_id: &str) -> Result<Option<UserDetails>> {
        let test = UserDetails::new(format!("test"));
        Ok(Some(test))
    }

    fn collection() -> Collection<UserDetails> { 
        MongoDbClient::get_collection(COLL_NAME)
    }

    
}