use serde_derive::{Serialize, Deserialize};



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Language { 
    #[default]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq,  Default)]
pub enum Theme { 
    AUTO,

    #[default]
    LIGHT,
    DARK
}