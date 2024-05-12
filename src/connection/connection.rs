use bb8_redis::{bb8::Pool, RedisConnectionManager};

/// Connects to a Redis instance and returns a connection pool.
///
/// # Arguments
///
/// * `uri` - The URI of the Redis instance.
///
/// # Returns
///
/// A `Pool` of `RedisConnectionManager` instances.
///
/// # Panics
///
/// This function will panic if it fails to create the connection pool.
///
/// # Example
///
/// ```rust
/// use your_module::connect;
///
/// #[tokio::main]
/// async fn main() {
///     let pool = connect("redis://localhost:6379/0".to_string()).await.unwrap();
///     // Use the pool to perform operations on the Redis instance
/// }
/// ```
///
/// # Implementation Details
///
/// This function uses the `bb8_redis` crate to create a connection pool to a Redis instance. It first creates a `RedisConnectionManager` instance using the provided URI, and then builds a connection pool using the `bb8::Pool::builder` method. The `QueueStrategy::Fifo` is used to manage the queue of connections.
///
/// # Errors
///
/// This function returns a `Result` containing a `Pool<RedisConnectionManager>` if successful, or an error if it fails to create the connection pool.
///
/// ```rust
/// use your_module::connect;
///
/// #[tokio::main]
/// async fn main() {
///     match connect("invalid_uri".to_string()).await {
///         Ok(pool) => {
///             // Use the pool to perform operations on the Redis instance
///         }
///         Err(err) => {
///             // Handle the error
///             println!("Error creating connection pool: {}", err);
///         }
///     }
/// }
/// ```
///
/// # Caveats
///
/// This function assumes that the `bb8_redis` crate is available and correctly configured. It also assumes that the provided URI is valid and that the Redis instance is reachable.
///
/// ```rust
/// use your_module::connect;
///
/// #[tokio::main]
/// async fn main() {
///     // This will panic because the URI is invalid
///     let pool = connect("invalid_uri".to_string()).await.unwrap();
/// }
/// ```
///
/// # Related Functions
///
/// * `bb8_redis::bb8::Pool::builder` - Builds a connection pool using the specified connection manager.
/// * `bb8_redis::bb8::QueueStrategy::Fifo` - Specifies the queue strategy to use for managing the connections in the pool.
///
/// ```rust
/// use your_module::connect;
///
/// #[tokio::main]
/// async fn main() {
///     let pool = connect("redis://localhost:6379/0".to_string()).await.unwrap();
///     // Use the pool to perform operations on the Redis instance
/// }
/// ```

#[allow(dead_code)]
pub async fn connect(uri: String) -> Pool<RedisConnectionManager> {
    let redis_manager = RedisConnectionManager::new(uri).unwrap();
    bb8_redis::bb8::Pool::builder().queue_strategy(bb8_redis::bb8::QueueStrategy::Fifo)
        .build(redis_manager)
        .await
        .unwrap()
}