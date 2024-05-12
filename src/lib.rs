mod connection;
mod routes;
use std::{collections::HashMap, sync::Arc};
use axum::Router;
use routes::create_routes;
use shuttle_runtime::SecretStore;
use socketioxide::SocketIo;
mod core;
mod models;
mod utils;
use routes::web_scoket::on_connect;
use tokio::sync::Mutex;

pub struct Store {
    #[allow(dead_code)]
    redis_uri: String,
}

use crate::{connection::connection::connect, utils::app_state};
pub async fn run(secrets: SecretStore ) -> Router {
    let redis: String = secrets.get("REDIS_URI").unwrap();
    let store = Store { redis_uri: redis.clone() };
    let r_client = connect(redis).await;
    let app_state_redis = app_state::AppState {
        redis: r_client,
        connections: Arc::new(Mutex::new(HashMap::new())),
        sockets: Arc::new(Mutex::new(HashMap::new()))
    };
    println!("Redis connected");
    let (layer, io) = SocketIo::builder().with_state(app_state_redis).with_state(store).build_layer();
    io.ns("/", on_connect);
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = create_routes(layer).await;
    app
    // axum::serve(listener, app).await.unwrap();
}
