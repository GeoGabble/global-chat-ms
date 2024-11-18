use std::{collections::HashMap, sync::Arc};
use redis::Client;
use tokio::sync::Mutex as TMutex;
// use crate::connection::connection::RedisPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub redis: Client,
    pub connections: Arc<TMutex<HashMap<String, String>>>,
    pub sockets: Arc<TMutex<HashMap<String, String>>>
}
