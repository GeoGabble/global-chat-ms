use bb8_redis::{bb8::Pool, RedisConnectionManager};

#[warn(dead_code)]
pub async fn connect() -> Pool<RedisConnectionManager> {
    let redis_manager = RedisConnectionManager::new("redis://default:j6KmRVGUqabDJPwxg9VMTcYX0V9lQIHj@redis-13084.c264.ap-south-1-1.ec2.redns.redis-cloud.com:13084").unwrap();
    bb8_redis::bb8::Pool::builder().queue_strategy(bb8_redis::bb8::QueueStrategy::Fifo)
        .build(redis_manager)
        .await
        .unwrap()
}
