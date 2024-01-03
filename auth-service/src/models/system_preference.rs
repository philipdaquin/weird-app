use super::{account::Account, enums::{lang::Language, theme::Theme}};


#[derive(Debug, Clone)]
pub struct SystemPreference { 
    id: String, 
    pub language: Language,
    pub theme: Theme, 
    //  
    //  Search engine index: 'noindex', 'nofollow', 'all'
    // 
    pub meta_index_status: String
}