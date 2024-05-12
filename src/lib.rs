mod connection;
mod routes;
use std::{collections::HashMap, sync::Arc};
use axum::Router;
use routes::create_routes;
use socketioxide::SocketIo;
mod core;
mod models;
mod utils;
mod logging;
use routes::web_scoket::on_connect;
use tokio::sync::Mutex;

use crate::{connection::connection::connect, utils::app_state};
pub async fn run() -> Router {

    
    let r_client = connect().await;
    let app_state_redis = app_state::AppState {
        redis: r_client,
        connections: Arc::new(Mutex::new(HashMap::new())),
        sockets: Arc::new(Mutex::new(HashMap::new()))
    };
    println!(" redis connected");

    let (layer, io) = SocketIo::builder().with_state(app_state_redis).build_layer();

    io.ns("/", on_connect);

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = create_routes(layer).await;
    app
    // axum::serve(listener, app).await.unwrap();
}
