use serde_derive::{Serialize, Deserialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language { 
    ENGLISH, 
    ITALIAN,
    SPANISH
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType { 
    TWITTER, 
    INSTRAGRAM,
    DISCORD,
    TIKTOK,
    SNAPCHAT,
    REDDIT,
    SPOTIFY,
    GITHUB,
    FIGMA,
    CUSTOM
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme { 
    AUTO,
    LIGHT,
    DARK
}