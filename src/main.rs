use anyhow::Result;
use axum::Extension;
use axum::routing::post;
use clap::Parser;
use tokio::sync::RwLock;

use std::sync::Arc;
use std::{collections::BTreeMap, net::SocketAddr};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use axum::{Router, routing::get};

use roll::models::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Poll API starting up...");

    let config = roll::models::AppConfiguration::parse();

    let bind_string = format!("0.0.0.0:{}", config.port);

    let app_state = Arc::new(RwLock::new(AppState {
        polls: BTreeMap::new(),
    }));

    let config = Arc::new(config);

    // build our application with routes
    let app = Router::new()
        .route("/", get(roll::routes::static_routes::get_index))
        .route("/index.html", get(roll::routes::static_routes::get_index))
        .route("/poll", get(roll::routes::static_routes::get_index))
        .route("/poll/:id", get(roll::routes::static_routes::get_index))
        .route("/bundle.js", get(roll::routes::static_routes::get_bundle))
        .route("/api/poll/:id", get(roll::routes::polls::get_poll))
        .route(
            "/api/poll/:id/options",
            get(roll::routes::polls::get_poll_options),
        )
        .route("/api/poll", post(roll::routes::polls::create_poll))
        .route("/api/poll/:id/vote", post(roll::routes::polls::vote_poll))
        .with_state(app_state)
        .layer(Extension(config));

    // run our app with hyper, listening globally on port 3000

    let listener = tokio::net::TcpListener::bind(bind_string.clone()).await?;
    info!("Poll API listening on http://{}", bind_string);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
