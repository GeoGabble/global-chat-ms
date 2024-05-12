use crate::models::message_model::LocationDto;
use crate::utils::app_state::AppState;
use crate::utils::failovers::retry::Failovers;
use bb8_redis::{bb8::PooledConnection, RedisConnectionManager};
use geohash::{neighbor, Direction};
use redis::geo::{Coord, RadiusOptions, RadiusOrder, RadiusSearchResult, Unit};
use redis::{AsyncCommands, RedisError};
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub async fn get_connected_users(client: AppState, user_id: String) -> HashSet<String> {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let users: HashSet<String> = pool
        .smembers(format!("connected:{}", user_id))
        .await
        .unwrap();
    users
}
pub async fn update_location(client: AppState, geo_hash: String, user_id: String) {
    println!("Updating");
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    // let pool_mut = Arc::new(Mutex::new(pool));
    let val: Result<Option<String>, redis::RedisError> =
        pool.get(format!("users:{}", user_id)).await;
    match val {
        Ok(value) => {
            if let None = value {
                println!("Inserting value");
                user_add_update(&mut pool, geo_hash, user_id).await;
            } else if let Some(valu) = value {
                if valu != geo_hash
                {
                    let failover = Failovers::new();
                    let client_clone = client.clone();
                    let id = user_id.clone();
                    let geo = geo_hash.clone();
                    let res = failover.retry(move || location_changed(
                        client_clone.clone(),
                        geo.clone(),
                        id.clone(),
                    ),5,Duration::from_secs(0)).await;

                    println!("{:?}",res);
                    // location_changed(
                    //     &mut pool,
                    //     value.get("state").unwrap().to_owned(),
                    //     value.get("country").unwrap().to_owned(),
                    //     user_id.clone(),
                    // )
                    // .await;
                    user_add_update(&mut pool, geo_hash, user_id).await;
                }
            }
        }
        Err(err) => {
            println!("E getting users:user_id: {:?}", err);
        }
    }
}

pub async fn check_buffer(client: AppState, user_id: String) -> bool {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let value: Result<Option<u64>, redis::RedisError> = pool.hget("buffer_states", &user_id).await;
    match value {
        Ok(value) => {
            if let Some(state) = value {
                // if state
                if state
                    < SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                {
                    println!("Buffer elapsed");
                    let now = SystemTime::now();
                    let up = now.checked_add(Duration::from_secs(5)).unwrap();
                    let _: () = pool
                        .hset(
                            "buffer_states",
                            &user_id,
                            up.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                        )
                        .await
                        .unwrap();
                    return false;
                } else {
                    println!("Buffer active");
                    return true;
                }
            } else {
                let now = SystemTime::now();
                let up = now.checked_add(Duration::from_secs(5)).unwrap();
                let _: () = pool
                    .hset(
                        "buffer_states",
                        &user_id,
                        up.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                    )
                    .await
                    .unwrap();
                eprintln!("User {} not found", user_id);
                return false;
            }
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error: {:?}", err);
            return false;
        }
    }
}

pub async fn update_lat_lng(client: AppState, geo_hash: String, message: LocationDto ) {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let _: () = pool
        .geo_add(
            format!("curLoc:{}", geo_hash),
            (
                Coord::lon_lat(message.longitude, message.latitude),
                &message.user_id,
            ),
        )
        .await
        .unwrap();
    let neighbours = vec![geo_hash.clone(),neighbor(&geo_hash, Direction::E).unwrap(),neighbor(&geo_hash, Direction::N).unwrap(),neighbor(&geo_hash, Direction::NE).unwrap(),neighbor(&geo_hash, Direction::NW).unwrap(),neighbor(&geo_hash, Direction::S).unwrap(),neighbor(&geo_hash, Direction::SE).unwrap(),neighbor(&geo_hash, Direction::SW).unwrap(),neighbor(&geo_hash, Direction::W).unwrap()];
    let mut ids = Vec::new();

    for nei in neighbours.iter() {
        let opts = RadiusOptions::default().with_dist().order(RadiusOrder::Asc);
        let mut id_ne: Vec<RadiusSearchResult> = pool.geo_radius(format!("curLoc:{}",nei), message.longitude, message.latitude, 5.0, Unit::Kilometers, opts).await.unwrap();
        ids.append(&mut id_ne);
    }
    println!("Nearby users: {:?}", ids.len());
    let _ : () =pool.del(format!("connected:{}", &message.user_id)).await.unwrap();

    for id in ids.iter() {
        if &id.name != &message.user_id {
            let _: () = pool
                .sadd(format!("connected:{}", &message.user_id), &id.name)
                .await
                .unwrap();
        }
    }
}

pub async fn location_changed(
    client: AppState,
    geo_hash: String,
    user_id: String,
)-> Result<(), RedisError>{
    let mut pool = client.redis.get().await.unwrap();
    let y: Result<(), RedisError>= pool
        .zrem(format!("curLoc:{}", geo_hash), user_id)
        .await;
    y
}

pub async fn user_add_update(
    pool: &mut PooledConnection<'_, RedisConnectionManager>,
    geo_hash: String,
    user_id: String,
) {
    let _ : String = pool.set(format!("users:{}",user_id), geo_hash).await.unwrap();
}
