use redis::{Commands, RedisError};
use crate::utils::app_state::AppState;
use super::location::location_changed;

pub async fn reset_user(user_id: String, client: AppState) {

    println!("Resetting user:{}", user_id);
    let clone = client.clone();
    let mut pool= client.redis;
    let val: Result<String, redis::RedisError> =
        pool.get(format!("users:{}", user_id));
    match val {
        Ok(value) => {
            tokio::spawn(location_changed(
               clone.clone(),
                value,
                user_id.clone(),
            ));
            tokio::spawn(drop_connected(clone.clone(), user_id.clone()));
            tokio::spawn(drop_loc(clone.clone(), user_id.clone()));
            tokio::spawn(drop_buffer(clone.clone(), user_id.clone()));
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub async fn drop_connected(client: AppState, user_id: String) -> Result<(), RedisError> {
    let mut pool = client.redis;
    let y: Result<(), RedisError> = pool.del(format!("connected:{}", &user_id));
    y
}
pub async fn drop_buffer(client: AppState, user_id: String) -> Result<(), RedisError> {
    let mut pool = client.redis;
    let y: Result<(), RedisError> = pool.hdel("buffer_states", &user_id);
    y
}
pub async fn drop_loc(client: AppState, user_id: String) -> Result<(), RedisError> {
    let mut pool = client.redis;
    let y: Result<(), RedisError> = pool.del(format!("users:{}", &user_id));
    y
}
