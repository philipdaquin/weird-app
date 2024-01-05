
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;
use mongodb::error::Error as MongoError;
use std::io;

#[derive(Debug, Error)]
pub enum ServerError { 
    #[error("Input / Output operation fails: {0:#?}")]
    IoError(#[source] io::Error), 

    #[error("Mongo Database Error: {0:#?}")]
    MongoDatabaseError(#[source] MongoError),

    #[error("Missing Item. Null Exception.")]
    NotFound
}

impl From<io::Error> for ServerError { 
    fn from(value: io::Error) -> Self {
        ServerError::IoError(value)
    }
}

impl From<MongoError> for ServerError {
    fn from(value: MongoError) -> Self {
        ServerError::MongoDatabaseError(value)
    }
}