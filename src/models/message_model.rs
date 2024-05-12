use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocationDto {
    pub user_id: String,
    pub latitude: f64,
    pub longitude: f64,
    // pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageDto {
    pub user_id: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoomConnections {
    pub message: MessageDto,
    pub receivers: HashSet<String>
}
