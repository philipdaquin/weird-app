use serde_derive::{Deserialize, Serialize};

use super::{account::Account, link_details::LinkDetails};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLinks { 
    id: String, 
    account: Account,
    links: Vec<LinkDetails>
}