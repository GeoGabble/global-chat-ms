use global_ms::run;
mod connection;
use shuttle_runtime::SecretStore;
use global_ms::authentication::auth_client::AuthClient;
use tonic::transport::Channel;

#[shuttle_runtime::main]
pub async fn axum (
    #[shuttle_runtime::Secrets] secrets: SecretStore
) -> shuttle_axum::ShuttleAxum {

    let auth_client: AuthClient<Channel> = AuthClient::connect("http://localhost:8088").await.unwrap();

    let router = run(secrets, auth_client).await;

    Ok(router.into())
}