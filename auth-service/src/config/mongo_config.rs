use mongodb::{Client, Collection, options::{ClientOptions, Credential}};
use once_cell::sync::OnceCell;
use crate::error::Result;
use lazy_static::lazy_static;

lazy_static! { 
    pub static ref MONGO_URI: String = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://username:password@mongo-db:27017".into());
    pub static ref MONGODB_USERNAME: String = std::env::var("MONGODB_USERNAME")
        .unwrap_or_else(|_| "username".into());
    pub static ref MONGODB_PASSWORD: String = std::env::var("MONGODB_PASSWORD")
        .unwrap_or_else(|_| "password".into());
}


#[derive(Debug, Clone)]
pub struct MongoDbClient(pub Client);

pub static MONGOCONN: OnceCell<MongoDbClient> = OnceCell::new();

/// Singleton implementation of MongoClient Connection 
#[inline]
pub(crate) fn get_mongo_client() -> &'static MongoDbClient { 
    MONGOCONN.get().expect("Missing Mongo Client")
}

impl From<Client> for MongoDbClient { 
    fn from(value: Client) -> Self {
        Self(value)
    }
}

impl MongoDbClient {  
    pub async fn establish_connection(uri: &str) -> Result<&'static Self> { 
        let mut client_options = ClientOptions::parse(uri)
            .await
            .expect("Failed to created MongoDB client");

        // Disabling Authentication for Development mode 
        let credential = Credential::builder()
            .username(MONGODB_USERNAME.to_string())
            .password(MONGODB_PASSWORD.to_string())
            .build();   
        client_options.credential = Some(credential);

        // Create a client
        let client = Client::with_options(client_options)
            .expect("Unable to establish connection");
    
        //  Store Current Session as a global variable
        let mongo = MongoDbClient::from(client);
        let _ = MONGOCONN.set(mongo);

        Ok(MONGOCONN.get().unwrap())
    }
    
    /// Returns the Instantiation of Collection 
    pub fn get_collection<T>(column_name: &str, db_name: &str) -> Collection<T> { 
        get_mongo_client().0.database(db_name).collection(column_name)
    }  
}