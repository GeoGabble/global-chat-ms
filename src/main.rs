use global_ms::run;
mod connection;
use shuttle_runtime::SecretStore;
#[shuttle_runtime::main]
pub async fn axum (
    #[shuttle_runtime::Secrets] secrets: SecretStore
) -> shuttle_axum::ShuttleAxum {

    let router = run(secrets).await;

    Ok(router.into())
}