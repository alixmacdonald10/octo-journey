use axum::{
    routing::{get, post},
    Router
};
use clap::{Arg, ArgAction, Command};
use tracing::{Level, event, instrument};
use tokio::{
    net::TcpListener,
    time::{sleep, Duration},

};


fn main() {
    let matches = Command::new("octo")
        .about("Basic test server")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("journey")
                .about("Start the test server")
                .arg(
                    Arg::new("address")
                        .short("a")
                        .long("address")
                        .help("Server address")
                        .env("OCTO_SERVER_ADDRESS")
                        .default_value("0.0.0.0")
                )
                .arg(
                    Arg::new("port")
                        .short("p")
                        .long("port")
                        .help("Server port")
                        .env("OCTO_SERVER_PORT")
                        .default_value("8080")
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("verbosity")
                        .short("v")
                        .help("Set the log level")
                        .required(false)
                        .env("OCTO_SERVER_VERBOSITY")
                )
        )
        .get_matches();

    let address = matches.get_one::<String>("address").unwrap();
    println!("Server address: {}", address);

    let port = matches.get_one::<u32>("port").unwrap();
    println!("Server port: {}", port);
    

    let level = match matches.get_one::<u8>("verbosity").expect("Count's are defaulted") {
        0 => {
            println!("Log level set at INFO");
            0
        },
        1 => {
            println!("Log level set at TRACE");
            1
        },
        _ => {
            println!("Log level set at DEBUG. You really don't trust that Octopus...");
            2
        }
    };


    psuedo_main(address, port);
}



#[tokio::main]
async fn psuedo_main(server_address: &String, server_port: &u32) {
    // TODO: set up logging and tracing
    // tracing_subscriber::fmt::init();

    // TODO: add shared mutable state and handle that in functionos
    // TODO: add body to post requests with keys etc and check
    // TODO: proper mietted error handling
    let app = Router::new()
        .route("/", get(root))
        .route("/lock", post(locker))
        .route("/unlock", post(unlocker));

    let listener = TcpListener::bind(format!("{}:{}", server_address, server_port.into()))
        .await
        .expect(format!("Failed to bind listener to server and port: {}:{}", server_address.as_str(), server_port.into()).as_str());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[instrument]
async fn root() -> &'static str {
    event!(Level::INFO, "You hit the root!");
    let x = "I am the root!";
    event!(Level::TRACE, "Returning root");
    x
}

#[instrument]
async fn locker() {
    event!(Level::INFO, "You are locking the octopus");
    sleep(Duration::from_millis(100)).await;
    event!(Level::TRACE, "After a brief fight you've locked the octopus")
}

#[instrument]
async fn unlocker() {
    event!(Level::INFO, "You are unlocking the octopus, BEWARE!");
    sleep(Duration::from_millis(300)).await;
    event!(Level::TRACE, "You have successfully unlocked the octopus, may god have mercy on your soul");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install ctrl+c handler!");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending:::<()>();

    tokio_select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
