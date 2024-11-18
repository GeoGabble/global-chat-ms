// use futures_util::Future;
// use redis::{ErrorKind, RedisError};
// use std::time::Duration;

// pub struct Failovers;

// impl Failovers {
//     pub fn new() -> Failovers {
//         Failovers
//     }

//     pub async fn retry<F, T>(
//         &self,
//         f: F,
//         max_retries: usize,
//         retry_delay: Duration,
//     ) -> Result<(), RedisError>
//     where
//         F: Fn() -> T + Send + 'static,
//         T: Future<Output = Result<(), RedisError>> + Send + 'static,
//     {
//         for _ in 0..max_retries {
//             let result = f().await;

//             if result.is_ok() {
//                 return result;
//             }

//             tokio::time::sleep(retry_delay).await;
//         }

//         Err(RedisError::from((ErrorKind::ResponseError, "Retry Failed")))
//     }
// }
