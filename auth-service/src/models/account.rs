use serde_derive::{Deserialize, Serialize};

use super::system_preference::SystemPreference;

/*

    Testing Simple Data Model    

*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Account { 
    id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub img_src: Option<String>, 
    pub email: Option<String>, 
    password: Option<String>, 
    
    //  
    // To be tested with Rauthy's functionalities 
    // 
    id_token: Option<String>,
    encrypted_token: Option<String>, 
    activation_key: Option<String>, 


    //
    //   
    //
    system_preference: SystemPreference
}

impl Account  { 
    pub fn new(email: &str) -> Self { 
        let mut account = Self::default();
        account.email.insert(email.to_string());

        return account;
    }
    pub fn get_id(&self) -> &String { 
        return &self.id
    }
}
