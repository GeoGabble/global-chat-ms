mod connection;
mod routes;
use axum::Router;
use routes::create_routes;
use shuttle_runtime::SecretStore;
use socketioxide::{
    extract::{SocketRef, State},
    handler::ConnectHandler,
    SocketIo,
};
use std::{collections::HashMap,sync::Arc};
mod core;
mod models;
mod utils;
use authentication::auth_client::AuthClient;
use authentication::VerificationRequest;
use routes::web_scoket::on_connect;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use utils::grpc_errors::grpc_error::GrpcError;
use crate::{connection::connection::connect, utils::app_state};

pub mod authentication {
    tonic::include_proto!("authentication");
}

/// This function is a middleware for authenticating socket connections using gRPC.
/// It extracts the user ID and token from the socket request headers, sends a verification request to the gRPC service,
/// and handles the response accordingly.
///
/// # Parameters
///
/// * `socket`: A reference to the socket that initiated the connection.
/// * `client`: A state containing an Arc-wrapped Mutex-protected AuthClient instance for making gRPC requests.
///
/// # Returns
///
/// * `Result<(), GrpcError>`: Returns Ok(()) if the authentication is successful, or an error if it fails.
///
/// # Errors
///
/// * `GrpcError::Unauthorized`: If the gRPC service returns an "error" status.
/// * `GrpcError::InternalError`: If the gRPC service returns an unexpected status or encounters an error.
/// * `GrpcError::HeadersMissing`: If the user ID or token headers are missing in the socket request.
async fn auth_middle(
    socket: SocketRef,
    client: State<Arc<Mutex<AuthClient<Channel>>>>,
) -> Result<(), GrpcError> {
    // Extract user ID and token from the socket request headers
    let request = &socket.req_parts().headers;
    let user_id = request
       .get("user_id")
       .map(|header| header.to_str().unwrap_or_default());
    let token = request
       .get("token")
       .map(|header| header.to_str().unwrap_or_default());

    // Log the extracted user ID and token
    println!("User ID: {:?}", user_id);
    println!("Token: {:?}", token);

    // Perform authentication if both user ID and token are present
    match (user_id, token) {
        (Some(user_id), Some(token)) => {
            // Create a gRPC request with the extracted user ID and token
            let request = tonic::Request::new(VerificationRequest {
                user_id: user_id.to_string(),
                token: token.to_string(),
            });

            // Send the gRPC request and await the response
            let response = client.lock().await.verify(request).await;

            // Handle the response
            match response {
                Ok(val) => {
                    let result = val.get_ref();
                    match result.status.as_str() {
                        "success" => {
                            println!("Success");
                            return Ok(());
                        }
                        "error" => {
                            println!("Failure");
                            return Err(GrpcError::Unauthorized);
                        }
                        _ => {
                            println!("Error");
                            return Err(GrpcError::InternalError);
                        }
                    }
                }
                Err(err) => {
                    println!("Error in grpc authentication: {:?}", err);
                    return Err(GrpcError::InternalError);
                }
            }
        }
        _ => return Err(GrpcError::HeadersMissing),
    }
}







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
pub async fn run(secrets: SecretStore, auth_client: AuthClient<Channel>) -> Router {
    // Retrieves the REDIS_URI secret from the SecretStore instance.
    let redis: String = secrets.get("REDIS_URI").unwrap();

    // Connects to the Redis server using the provided URI.
    let r_client = connect(redis).await;

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
        .with_state(app_state_redis)
        // .with_state(store)
        .with_state(Arc::new(Mutex::new(auth_client)))
        .build_layer();

    // Sets up the Socket.IO namespace "/" with the provided on_connect handler, which includes the auth_middle function.
    io.ns("/", on_connect.with(auth_middle));

    // Creates the application's routes using the provided layer and asynchronously awaits the result.
    let app = create_routes(layer).await;

    // Returns the created Router instance, representing the application's routes.
    app
}
