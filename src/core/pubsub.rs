use futures::StreamExt;
use socketioxide::SocketIo;
// use redis::Commands;
use crate::{core::message_transit::transit_message, models::pub_sub_model::MessagePayload, utils::app_state::AppState};

pub async fn redis_subscribe(io: SocketIo,app_state: AppState) {
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
                    println!("Message received from payload: {:?}", msg);
                    if let Ok(payload) = msg.get_payload::<String>() {
                        let parsed_payload: MessagePayload = serde_json::from_str(&payload).unwrap();
                        transit_message(parsed_payload, app_state.clone(), io.clone()).await;
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
