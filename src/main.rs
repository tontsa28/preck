use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, Server};
use once_cell::sync::Lazy;
use std::{
    env::{set_var, var},
    fs::create_dir,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};
use tracing::{info, warn};

static ADDRESS: Lazy<IpAddr> = Lazy::new(|| {
    // Check if ADDRESS is set
    match var("ADDRESS") {
        Ok(addr) => {
            // Parse ADDRESS into an IpAddr
            addr.parse::<IpAddr>()
                .expect("ADDRESS is not a valid IP address")
        }
        Err(_) => {
            // Set ADDRESS to localhost(127.0.0.1)
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
                .expect("PORT must be a 16-bit unsigned integer")
        }
        Err(_) => {
            // Set PORT to 8080
            set_var("PORT", "8080");

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

            // Check if PID_FILE ends in a '/'
            if !path.ends_with("/") {
                panic!("PID_FILE should end in a '/'");
            }

            // Check if PID_FILE exists
            if !path.exists() {
                if let Err(err) = create_dir(path) {
                    panic!("PID_FILE does not exist: {err}");
                }
            }

            path.to_path_buf()
        }
        Err(_) => {
            // Panic because PID_FILE is required to run the program
            panic!("PID_FILE should be set");
        }
    }
});

async fn check() -> impl IntoResponse {
    // Get the pid file path
    let pid = &*PID_FILE;

    // Check if the pid file exists
    if pid.exists() {
        // Return code 200 OK
        info!("Process running");
        StatusCode::OK.into_response()
    } else {
        // Return code 404 NOT FOUND
        warn!("Process not running");
        StatusCode::NOT_FOUND.into_response()
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

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

    Ok(())
}
