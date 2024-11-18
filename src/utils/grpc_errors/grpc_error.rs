// #[derive(Debug)]
// pub enum GrpcError {
//     HeadersMissing,
//     InternalError,
//     Unauthorized,
// }

// impl std::fmt::Display for GrpcError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             GrpcError::HeadersMissing => write!(f, "User ID or Token missing in request headers"),
//             GrpcError::InternalError => write!(f, "Internal server error"),
//             GrpcError::Unauthorized => write!(f, "Unauthorized"),
//         }
//     }
// }