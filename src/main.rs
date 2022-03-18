#[macro_use] extern crate rocket;
use std::path::Path;
use rocket::http::Status;
use rocket::routes;

#[get("/")]
async fn check() -> Status {
    let pid = dotenv::var("PIDFILE").expect("ERROR: Pidfile missing");
    if Path::new(&pid).exists() {
        println!("TS3 server running");
        Status::Ok
    }
    else {
        println!("TS3 server not running");
        Status::NotFound
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("ERROR: Failed to load .env file");
    rocket::build().mount("/", routes![check]).launch().await.unwrap();
}
