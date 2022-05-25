#[macro_use] extern crate rocket;
use std::path::Path;
use rocket::http::Status;
use rocket::routes;
use tracing::info;

// Root endpoint
#[get("/")]
async fn check() -> Status {
    // Get pid file path from .env file
    let pid = dotenv::var("PIDFILE").expect("ERROR: PIDFILE path missing");

    // If pid file exists
    if Path::new(&pid).exists() {
        // Return code 200
        info!("Process running");
        Status::Ok
    }
    // If pid file does not exist
    else {
        // Return code 404
        info!("Process not running");
        Status::NotFound
    }
}

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // Set root endpoint to check function and launch Rocket
    let _rocket = rocket::build().mount("/", routes![check]).launch().await?;
    Ok(())
}