use super::{account::Account, link_details::LinkDetails, user_links::UserLinks};

#[derive(Debug, Clone)]
pub struct UserDetails { 
    id: String, 
    pub name: String,
    pub account: Account,
    pub bio: String,
    pub avatar_url: String, 
    pub avatar_2x_url: String,
    pub meta_title: String, 
    pub meta_description: String, 
    pub meta_author: String, 
    pub meta_keywords: Vec<String>,
    pub links: UserLinks
}

impl UserDetails {
    pub fn get_id(&self) -> &str { 
        return &self.id;
    }
}