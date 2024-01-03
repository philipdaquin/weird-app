use super::system_preference::SystemPreference;

/*

    Testing Simple Data Model    

*/
#[derive(Debug, Clone)]
pub struct Account { 
    id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub img_src: String, 
    pub email: String, 
    pub password: String, 
    
    //  
    // To be tested with Rauthy's functionalities 
    // 
    pub id_token: String,
    pub encrypted_token: String, 
    pub activation_key: String, 


    //
    //   
    //
    pub system_preference: SystemPreference
}