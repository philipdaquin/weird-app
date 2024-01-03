use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};

use std::fs::File;
use std::io::Write;
use dotenv::dotenv;
use crate::config::mongo_config::MongoDbClient;



pub async fn new_server(port: u32) -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    // Server attribute 
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let address = format!("{}:{}", host, port);


    // Database connection 
    let url = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://mongo-db:27017".into());
    // let mongo_client = MongoDbClient::establish_connection(&url)
    //     .await
    //     .expect("Missing connection URI to MongoDb server");




    log::info!("🚀 Starting HTTP server on port 'http://{}:{}' ", host, port);
    log::info!("📢 Query at https://studio.apollographql.com/dev");
    
    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(mongo_client.clone()))
            // .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // .wrap(TracingLogger::default())
    })
    .workers(2)
    .bind(address)?
    .run()
    .await
}