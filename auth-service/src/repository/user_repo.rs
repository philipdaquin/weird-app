use async_trait::async_trait;

use crate::models::user_details::UserDetails;
use crate::error::Result;

#[async_trait]
pub trait UserInterface { 
    async fn get_user(_id: &str) -> Result<UserDetails>;
}

#[derive(Debug)]
pub struct UserRepository;


#[async_trait]
impl UserInterface for UserRepository { 

    async fn get_user(_id: &str) -> Result<UserDetails> {
        let test = UserDetails::new(format!("test"));
        Ok(test)
    }
}