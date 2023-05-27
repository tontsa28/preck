use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server,
};
use once_cell::sync::Lazy;
use std::{
    env::{set_var, var},
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

/* Handle environment variables */
static ADDRESS: Lazy<IpAddr> = Lazy::new(|| {
    // Check if ADDRESS is set
    match var("ADDRESS") {
        Ok(addr) => {
            // Parse ADDRESS into an IpAddr
            addr.parse::<IpAddr>()
                .expect("ADDRESS should be a valid IP address")
        }
        Err(_) => {
            // Set ADDRESS to localhost (127.0.0.1)
            set_var("ADDRESS", "127.0.0.1");

            // Get ADDRESS & parse it into an IpAddr
            var("ADDRESS")
                .expect("ADDRESS should be set")
                .parse::<IpAddr>()
                .expect("ADDRESS should be a valid IP address")
        }
    }
});
static PORT: Lazy<u16> = Lazy::new(|| {
    // Check if PORT is set
    match var("PORT") {
        Ok(port) => {
            // Parse PORT into a 16-bit unsigned integer
            port.parse::<u16>()
                .expect("PORT should be a 16-bit unsigned integer")
        }
        Err(_) => {
            // Set PORT to 8080
            set_var("PORT", "8081");

            var("PORT")
                .expect("PORT should be set")
                .parse::<u16>()
                .expect("PORT should be a 16-bit unsigned integer")
        }
    }
});
static PID_FILE: Lazy<PathBuf> = Lazy::new(|| {
    match var("PID_FILE") {
        Ok(path) => {
            let path = Path::new(&path);

            path.to_path_buf()
        }
        Err(_) => {
            // Panic because PID_FILE is required to run the program
            panic!("PID_FILE should be set");
        }
    }
});

async fn check() -> Response {
    if PID_FILE.exists() {
        // Return code 200 OK
        StatusCode::OK.into_response()
    } else {
        // Return code 404 NOT FOUND
        StatusCode::NOT_FOUND.into_response()
    }
}

/* Use only a single worker thread due to the only task being very lightweight */
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    // Initialize the logging system
    let format = tracing_subscriber::fmt::format()
        .compact()
        .with_target(false);
    tracing_subscriber::fmt()
        .event_format(format)
        .finish()
        .init();

    // Log the environment variables being used
    info!("Set ADDRESS={}", *ADDRESS);
    info!("Set PORT={}", *PORT);
    info!("Set PID_FILE={}", PID_FILE.display());

    // Create the socket & the router
    let socket = SocketAddr::new(*ADDRESS, *PORT);
    let router = Router::new().route("/", get(check));

    // Start the web server
    Server::bind(&socket)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to catch the SIGINT signal")
        })
        .await
        .expect("Failed to start the axum server");
}
