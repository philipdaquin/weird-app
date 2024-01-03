use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};

use std::fs::File;
use std::io::Write;
use dotenv::dotenv;


pub async fn new_server(port: u32) -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));




    log::info!("🚀 Starting HTTP server on port 'http://0.0.0.0:{}' ", port);
    log::info!("📢 Query at https://studio.apollographql.com/dev");
    
    HttpServer::new(move || {
        App::new()
            // .app_data(schema.clone())
            // .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // .wrap(TracingLogger::default())
    })
    .workers(2)
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}