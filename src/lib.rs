mod connection;
mod routes;
use axum::Router;
use routes::create_routes;
use socketioxide::SocketIo;
use core::pubsub::redis_subscribe;
// use core::pubsub::redis_subscribe;
use std::{collections::HashMap,sync::Arc};
mod core;
mod models;
mod utils;
use routes::web_scoket::on_connect;
use tokio::{sync::Mutex, task};
use crate::{connection::connection::connect, utils::app_state};



/// Runs the application, initializing the necessary components and setting up the Socket.IO server.
///
/// # Parameters
///
/// * `secrets`: A SecretStore instance containing the necessary secrets for the application.
/// * `auth_client`: An instance of the AuthClient for making gRPC requests.
///
/// # Returns
///
/// * `Router`: A Router instance representing the application's routes.
///
/// # Errors
///
/// This function does not return any errors. If an error occurs during the initialization process, it will be logged and the application will continue to run.
pub async fn run() -> Router {
    // Retrieves the REDIS_URI secret from the SecretStore instance.
    let redis: String = "redis://default:pDCYrBDY8fyzQZAeHPwLVzCiPPPsM37l@redis-16280.c212.ap-south-1-1.ec2.redns.redis-cloud.com:16280".to_string();

    // Connects to the Redis server using the provided URI.
    let r_client = connect(redis).await;

    // let x = redis::Client::open(redis).unwrap();

    // Creates an instance of the AppState struct, initializing the Redis and connection state.
    let app_state_redis = app_state::AppState {
        redis: r_client,
        connections: Arc::new(Mutex::new(HashMap::new())),
        sockets: Arc::new(Mutex::new(HashMap::new())),
    };

    // Prints a message indicating that the Redis server has been successfully connected.
    println!("Redis connected");

    // Initializes a new Socket.IO layer, setting the provided AppState instance as the state.
    let (layer, io) = SocketIo::builder()
        .with_state(app_state_redis.clone())
        .build_layer();

    // Sets up the Socket.IO namespace "/" with the provided on_connect handler, which includes the auth_middle function.
    io.ns("/", on_connect);

    task::spawn(redis_subscribe(io,app_state_redis.clone()));

    // Creates the application's routes using the provided layer and asynchronously awaits the result.
    let app = create_routes(layer).await;

    // Returns the created Router instance, representing the application's routes.
    app
}
