use async_trait::async_trait;
use mongodb::{Collection, bson::doc};
use crate::{models::account::Account, error::{Result, self}, config::mongo_config::MongoDbClient};
use super::RepositoryInterface;

const COLL_NAME: &str = "account";

#[derive(Debug)]
pub struct AccountRepository;


#[async_trait]
impl RepositoryInterface<Account, String> for AccountRepository { 
    async fn get_one(_id: String) -> Result<Option<Account>> {
        let collection = AccountRepository::collection();
        let filter = doc! {"_id": &_id};
        
        return collection.find_one(filter, None)
            .await
            .map(|a| a)
            .map_err(|_| error::ServerError::NotFound);
    }
    async fn save(account: Account) -> Result<Account> {
        let collection = Self::collection();
        
        let insert_res = collection
            .insert_one(account, None)
            .await?;
        
        let filter = doc! {"_id": &insert_res.inserted_id};

        return collection
            .find_one(filter, None)
            .await?
            .ok_or(error::ServerError::NotFound)
    }
    async fn delete(_id: String) -> Result<Option<Account>> {
        let collection = AccountRepository::collection();
        
        let filter = doc!{"_id": _id};

        return collection.find_one_and_delete(filter, None)
            .await
            .map(|f| f)
            .map_err(|e| error::ServerError::MongoDatabaseError(e));
    }
    async fn partial_update(new_account: &Account) -> Result<Option<Account>> {

        let collection = Self::collection();
        let id = new_account.get_id();
        let filter = doc! {"_id": id};

        let mut update_doc = doc!{};

        if let Some(update) = &new_account.first_name { 
            update_doc.insert("first_name", update);
        }
        if let Some(update) = &new_account.last_name { 
            update_doc.insert("last_name", update);
        }
        if let Some(update) = &new_account.email { 
            update_doc.insert("email", update);
        }
        if let Some(update) = &new_account.img_src { 
            update_doc.insert("img_src", update);
        }

        let bson_doc = bson::to_document(&update_doc)?;
        let update = doc! {"$set" : bson_doc};

        let _ = collection.update_one(filter, update, None).await?;

        return Self::get_one(id.to_string())
            .await
            .map(|res| res)
        
    }
    fn collection() -> Collection<Account> { 
        MongoDbClient::get_collection(COLL_NAME)
    }
}