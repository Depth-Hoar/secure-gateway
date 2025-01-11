// // Logging and monitoring logic

// use axum::{
//   http::Request,
//   middleware::{from_fn, Next},
//   response::Response,
// };
// use tracing::info;

// /// A simple logging middleware using tracing.
// /// Logs method, path, user info, IP, etc.
// pub async fn request_logger<B>(req: Request<B>, next: Next<B>) -> Response {
//   let method = req.method().clone();
//   let path = req.uri().path().to_string();
//   let remote_addr = req
//       .extensions()
//       .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
//       .map(|c| c.0)
//       .unwrap_or_else(|| "unknown:0".parse().unwrap());

//   info!("Incoming request: {} {} from {}", method, path, remote_addr);

//   let res = next.run(req).await;

//   let status = res.status();
//   info!("Response status: {}", status);

//   res
// }
