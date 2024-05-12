use axum::{routing::get, Router};
use socketioxide::layer::SocketIoLayer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
pub mod web_scoket;

pub async fn create_routes(layer: SocketIoLayer) -> Router {
    Router::new()
        .route("/location", get(|| async { "Hello, World!" }))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()).layer(layer))
}
