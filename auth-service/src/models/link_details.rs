use super::account::Account;
use super::enums::LinkType;


#[derive(Debug, Clone)]
pub struct LinkDetails { 
    id: String,
    pub account: Account,
    pub name: String, 
    pub link: String, 
    pub custom_logo: String, 
    pub link_type: LinkType
}