

#[derive(Debug, Clone)]
pub enum Language { 
    ENGLISH, 
    ITALIAN,
    SPANISH
}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Theme { 
    AUTO,
    LIGHT,
    DARK
}