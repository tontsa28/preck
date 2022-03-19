#[macro_use] extern crate rocket;
use std::path::Path;
use rocket::http::Status;
use rocket::routes;

// Root endpoint
#[get("/")]
async fn check() -> Status {
    // Get pid file path from .env file
    let pid = dotenv::var("PIDFILE").expect("ERROR: PIDFILE path missing");

    // If pid file exists
    if Path::new(&pid).exists() {
        // Return code 200
        println!("TS3 server running");
        Status::Ok
    }
    // If pid file does not exist
    else {
        // Return code 404
        println!("TS3 server not running");
        Status::NotFound
    }
}

// Main function
#[tokio::main]
async fn main() {
    // Set root endpoint to check function and launch Rocket
    rocket::build().mount("/", routes![check]).launch().await.unwrap();
}