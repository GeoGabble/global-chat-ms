use std::time::SystemTime;

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
pub struct AuthDto {
    pub user_id: String,
    pub token: String
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendMessageDto{
    pub user_id: String,
    pub message: String,
    pub timestamp: SystemTime
}
