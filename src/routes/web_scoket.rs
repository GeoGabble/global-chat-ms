use crate::core::location::{check_buffer, get_connected_users, update_lat_lng, update_location};
use crate::core::reset::reset_user;
use crate::models::message_model::{LocationDto, MessageDto, SendMessageDto};
use crate::models::pub_sub_model::MessagePayload;
use crate::utils::app_state::AppState;
use geohash::{encode, neighbor, Coord, Direction};
use redis::Commands;
use socketioxide::extract::{Data, SocketRef, State};


pub async fn on_connect(socket: SocketRef) {
    println!("Socket connected: {}", socket.id);

    socket.on("location", |socket: SocketRef, Data::<LocationDto>(data), client: State<AppState>| async move {
        println!("Updating location");
        let mut soc = client.sockets.lock().await;
        soc.insert(socket.id.to_string(), data.user_id.clone());
        let geo_hash = encode(Coord {x: data.latitude, y: data.longitude}, 5usize).unwrap();
        let mut map = client.connections.lock().await;
        println!("{}",geo_hash);
        // if let Some(hash) = map.get(&data.user_id) {
        //     if *hash != geo_hash {
        //         remove_old_hash(client.clone(), data.user_id.clone(), hash.to_string());
        //     }
        // }
        map.insert(data.user_id.clone(), geo_hash.clone());
        let update_location_task = tokio::spawn(update_location(
            client.clone(),
            geo_hash.clone(),
            data.user_id.clone(),
        ));
        let update_lat_lng_task = tokio::spawn(update_lat_lng(client.clone(), geo_hash.clone(), data.clone()));
        
        // Check the buffer only if the update_location and update_lat_lng tasks have completed
        if !check_buffer(client.clone(), data.user_id.clone()).await {
            // Await the completion of both tasks
            let _ = tokio::try_join!(update_location_task, update_lat_lng_task);
            // let neighbours = vec![geo_hash.clone(),neighbor(&geo_hash, Direction::E).unwrap(),neighbor(&geo_hash, Direction::N).unwrap(),neighbor(&geo_hash, Direction::NE).unwrap(),neighbor(&geo_hash, Direction::NW).unwrap(),neighbor(&geo_hash, Direction::S).unwrap(),neighbor(&geo_hash, Direction::SE).unwrap(),neighbor(&geo_hash, Direction::SW).unwrap(),neighbor(&geo_hash, Direction::W).unwrap()];
            let _ = socket.leave_all();
            let _ =  socket.join(geo_hash.clone());
        }
    });

    socket.on_disconnect(|socket: SocketRef, client: State<AppState>| async move{
        println!("Socket disconnected: {}", socket.id);

        let mut soc = client.sockets.lock().await;
        let mut map = client.connections.lock().await;
        {
            if let Some(id) = soc.get(&socket.id.to_string()) {
                reset_user(id.to_string(), client.clone()).await;
                map.remove(id);
            }
        }
        soc.remove(&socket.id.to_string());
    });

    socket.on("message", |socket: SocketRef, Data::<MessageDto>(data), client: State<AppState>| async move {
        println!("Message received: {:?}", data.message);
        let mut conn = client.redis.clone();
        let connections = get_connected_users(client.clone(), data.user_id.clone()).await;
        println!("{}", connections.len());
        let map = client.connections.lock().await;
        let geo_hash = map.get(&data.user_id);
        println!("{:?}",geo_hash);


        if let Some(geo_hash) = geo_hash {

            let payload = MessagePayload{
                user_id: data.user_id.clone(),
                message: data.message.clone(),
                geo_hash: geo_hash.clone()
            };

            let json_payload = serde_json::to_string(&payload).unwrap();
            println!("{}", json_payload);


            let publish : Result<(), redis::RedisError> = conn.publish("messages", json_payload);
            match publish {
                Ok(_) => {
                    let neighbours = vec![geo_hash.to_string(),neighbor(&geo_hash, Direction::E).unwrap(),neighbor(&geo_hash, Direction::N).unwrap(),neighbor(&geo_hash, Direction::NE).unwrap(),neighbor(&geo_hash, Direction::NW).unwrap(),neighbor(&geo_hash, Direction::S).unwrap(),neighbor(&geo_hash, Direction::SE).unwrap(),neighbor(&geo_hash, Direction::SW).unwrap(),neighbor(&geo_hash, Direction::W).unwrap()];
                    let sockets = socket.to(neighbours).sockets().unwrap();
                    for sockeet in sockets.iter() {
                        let soc_map = client.sockets.lock().await;
                        let user_id = soc_map.get(&sockeet.id.to_string());
                        if connections.contains(user_id.unwrap()) {
                            let _ = sockeet.emit("receive_message", SendMessageDto{
                                user_id: data.user_id.clone(),
                                timestamp: std::time::SystemTime::now(),
                                message: data.message.clone()
                            });
                        }
                    }
                },
                Err(e) => {
                    println!("Error publishing message: {}", e);
                }
            }
        }
        
    });
}