use geohash::{neighbor, Direction};
use socketioxide::SocketIo;

use crate::{models::{message_model::SendMessageDto, pub_sub_model::MessagePayload}, utils::app_state::AppState};

use super::location::get_connected_users;

pub async fn transit_message(payload: MessagePayload, client: AppState, socket: SocketIo) {
    let geo_hash = payload.geo_hash;
    let user_id = payload.user_id;
    let message = payload.message;
    let connections = get_connected_users(client.clone(), user_id.clone()).await;
    let neighbours = vec![
        geo_hash.to_string(),
        neighbor(&geo_hash, Direction::E).unwrap(),
        neighbor(&geo_hash, Direction::N).unwrap(),
        neighbor(&geo_hash, Direction::NE).unwrap(),
        neighbor(&geo_hash, Direction::NW).unwrap(),
        neighbor(&geo_hash, Direction::S).unwrap(),
        neighbor(&geo_hash, Direction::SE).unwrap(),
        neighbor(&geo_hash, Direction::SW).unwrap(),
        neighbor(&geo_hash, Direction::W).unwrap(),
    ];
    let sockets = socket.to(neighbours).sockets().unwrap();
    for sockeet in sockets.iter() {
        let soc_map = client.sockets.lock().await;
        let userid = soc_map.get(&sockeet.id.to_string());
        if connections.contains(userid.unwrap()) {
            let _ = sockeet.emit("receive_message", SendMessageDto{
                user_id: user_id.clone(),
                timestamp: std::time::SystemTime::now(),
                message: message.clone()
            });
        }
    }
}
