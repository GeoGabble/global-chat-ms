use futures::StreamExt;
// use redis::Commands;
use crate::utils::app_state::AppState;

pub async fn redis_subscribe(app_state: AppState) {
    let conn = app_state.redis.clone();
    loop {
        match conn.get_async_pubsub().await {
            Ok(mut pubsub) => {
                // println!("Pubsub connected");

                if let Err(err) = pubsub.subscribe("messages").await {
                    println!("Subscription failed: {:?}", err);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
                // println!("Subscribed to 'messages' channel");

                let mut pubsub_stream = pubsub.on_message();

                while let Some(msg) = pubsub_stream.next().await {
                    println!("Message received: {:?}", msg);
                    if let Ok(payload) = msg.get_payload::<String>() {
                        println!("Payload: {}", payload);
                    } else {
                        println!("Failed to get payload");
                    }
                }
                println!("Stream ended. Attempting to reconnect...");
            }
            Err(err) => {
                println!("Failed to connect to pubsub: {:?}", err);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
