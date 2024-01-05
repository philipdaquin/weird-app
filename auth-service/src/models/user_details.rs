use serde_derive::{Serialize, Deserialize};

use super::{account::Account, link_details::LinkDetails, user_links::UserLinks};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDetails { 
    id: String, 
    pub name: String,
    pub account: Option<Account>,
    pub bio: String,
    pub avatar_url: String, 
    pub avatar_2x_url: String,
    pub meta_title: String, 
    pub meta_description: String, 
    pub meta_author: String, 
    pub meta_keywords: Vec<String>,
    pub links: Option<UserLinks>
}

impl UserDetails {
    pub fn get_id(&self) -> &str { 
        return &self.id;
    }

    pub fn new(_id: String) -> Self { 
        let mut new_user = UserDetails::default();
        new_user.id = _id; 
        
        return new_user
    }
}