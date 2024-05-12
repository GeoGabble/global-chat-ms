use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex as TMutex};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
// use crate::connection::connection::RedisPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub redis: Pool<RedisConnectionManager>,
    pub connections: Arc<TMutex<HashMap<String, String>>>,
    pub sockets: Arc<TMutex<HashMap<String, String>>>
}
