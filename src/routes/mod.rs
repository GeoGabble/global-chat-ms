use axum::{routing::get, Router};
use socketioxide::layer::SocketIoLayer;
pub mod web_scoket;

pub async fn create_routes(layer: SocketIoLayer) -> Router {
    Router::new()
        .route("/location", get(|| async { "Hello, World!" }))
        .layer(layer)
}
