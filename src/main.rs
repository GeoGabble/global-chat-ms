use global_ms::run;
mod connection;

#[tokio::main]
async fn main () {
    let router = run().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}