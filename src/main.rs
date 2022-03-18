use std::path::Path;
use rocket::http::Status;
use rocket::routes;

#[get("/")]
async fn check(pid: &String) -> Status {
    if Path::new(pid).exists() {
        println!("TS3 server running");
        Status::Ok
    }
    else {
        println!("TS3 server not running");
        Status::NotFound
    }
}

async fn main() {
    dotenv::dotenv().expect("ERROR: Failed to load .env file");
    let port = dotenv::var("PORT").expect("ERROR: Port missing");
    let pid = dotenv::var("PIDFILE").expect("ERROR: Pidfile missing");

    rocket::ignite().mount("/", routes(check(&pid)).launch());
}
