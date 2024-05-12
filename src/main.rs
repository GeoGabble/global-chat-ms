use global_ms::run;
mod connection;
mod logging;

// use kafka::producer::Record;
// use logging::kafka_consumer::create_producer;

#[shuttle_runtime::main]
pub async fn axum (
) -> shuttle_axum::ShuttleAxum {
    // kafka_consumer().await;
    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Migrations failed :(");

    // Use secrets for anything that needs them

    let router = run().await;

    Ok(router.into())
}

// #[tokio::main]
// async fn main() {
//     // tracing_subscriber::fmt()
//     //     .with_max_level(tracing::Level::DEBUG)
//     //     .with_test_writer()
//     //     .init();
//     // let r_client = connect_redis();
//     // match r_client {
//     //     Ok(mut con) => {
//     //         println!("Connected to redis server");
//     //         let val: Result<HashMap<String,String>, redis::RedisError> = con.hgetall(format!("users:12356"));
//     //         println!("{:?}",val);
//     //         // let mut redis_client_mutex: Arc<Connection> = Arc::new(con);
//     //         run(con).await;
//     //     },
//     //     Err(err)=> {
//     //         println!("{:?}", err);
//     //     }
//     // }
//     run().await;
// }
