use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub user_id: String,
    pub geo_hash: String,
    pub message: String,
}
