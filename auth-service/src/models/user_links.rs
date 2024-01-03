use super::{account::Account, link_details::LinkDetails};


#[derive(Debug, Clone)]
pub struct UserLinks { 
    id: String, 
    account: Account,
    links: Vec<LinkDetails>
}