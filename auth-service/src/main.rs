use auth_service::server::new_server;
use thiserror::Error;
use auth_service::error::Result;

#[actix_web::main]
async fn main() -> Result<()> { 
    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(4001);
        
    new_server(port)
        .await
        .map_err(Into::into)
}