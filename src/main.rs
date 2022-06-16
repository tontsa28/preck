use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use std::path::Path;
use tracing::{info, warn};

// Root endpoint
#[get("/")]
async fn check() -> impl Responder {
    // Get pid file variable specified in the .env file
    let pid = dotenv::var("PID_FILE").expect("ERROR: PID_FILE path missing");

    // Check if the pid file exists
    if Path::new(&pid).exists() {
        // Return code 200 OK
        info!("Process running");
        HttpResponse::Ok()
    } else {
        // Return code 404 NOT FOUND
        warn!("Process not running");
        HttpResponse::NotFound()
    }
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // Get address and port variables specified in the .env file
    let address = dotenv::var("ADDRESS").expect("ERROR: Address missing");
    let port = dotenv::var("PORT").expect("ERROR: Port missing").parse::<u16>().unwrap();

    // Start the web server
    HttpServer::new(|| App::new().service(check)).bind((address, port))?.run().await
}