mod services;
mod utils;

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use clap::{value_parser, Arg, ArgAction, Command};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::SharedState;

use crate::services::*;

fn main() {
    let matches = Command::new("octo-journey")
        .about("Basic test server")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .help("Server address")
                .env("OCTO_SERVER_ADDRESS")
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Server port")
                .env("OCTO_SERVER_PORT")
                .default_value("8080")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .help("Set the log level")
                .required(false)
                .env("OCTO_SERVER_VERBOSITY")
                .action(ArgAction::Count),
        )
        .get_matches();

    // unwraps are fine as Clap has validated the inputs already
    let address = matches.get_one::<String>("address").unwrap().to_owned();
    let port = matches.get_one::<u32>("port").unwrap().to_string();
    let log_level = match matches
        .get_one::<u8>("verbosity")
        .expect("Count's are defaulted")
    {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    // set both package and tower tracing to log level
    let tracing_env_var = format!(
        "{}={},tower_http={}",
        env!("CARGO_PKG_NAME").replace("-", "_"),
        log_level,
        log_level
    );

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_env_var.into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
    tracing::info!("Server address: {}", address);
    tracing::info!("Server port: {}", port);
    tracing::info!("Log level: {}", log_level);

    psuedo_main(address, port);
}

#[tokio::main]
async fn psuedo_main(server_address: String, server_port: String) {
    let shared_state = SharedState::default();

    // TODO: proper mietted error handling
    let app = Router::new()
        .route("/", get(root))
        .route(
            "/v1/spot-check",
            get(v1::spot_check).with_state(Arc::clone(&shared_state)),
        )
        .route(
            "/v1/capture",
            post(v1::capture).with_state(Arc::clone(&shared_state)),
        )
        .route(
            "/v1/tag",
            post(v1::tag).with_state(Arc::clone(&shared_state)),
        )
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(format!("{}:{}", server_address, server_port))
        .await
        .expect(
            format!(
                "Failed to bind listener to server and port: {}:{}",
                server_address, server_port
            )
            .as_str(),
        );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    event!(Level::INFO, "Root route hit!");
    Html("<h1>Octo-Journey is up and running!</h1>")
}

async fn handler_404() -> impl IntoResponse {
    event!(Level::INFO, "Route not found!");
    (
        StatusCode::NOT_FOUND,
        "Route not found. Maybe some nasty octopus moved it...",
    )
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
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {tracing::event!(Level::WARN, "Gracefully shutting down")},
        _ = terminate => {tracing::event!(Level::ERROR, "Violently shutting down. Use CTRL+C to gracefully shutdown")},
    }
}
