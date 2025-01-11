// // Reverse proxy and route definitions

// use axum::{
//   extract::{Extension, Path},
//   http::{Request, StatusCode, Response as AxumResponse},
//   response::{IntoResponse, Response},
//   body::{Body as AxumBody, Bytes, HttpBody, boxed, BoxBody},
//   Json,
// };
// use serde::Deserialize;
// use std::sync::Arc;
// use tokio::sync::RwLock;
// use hyper::Body;
// use futures_util::stream::TryStreamExt;

// use crate::{
//   auth::{generate_jwt, Claims},
//   access::{user_has_access, Role},
//   AppState,
// };

// /// Login request payload
// #[derive(Deserialize)]
// pub struct LoginRequest {
//   email: String,
//   // In real scenario, also password / MFA token, etc.
// }

// /// Example login endpoint that issues JWT tokens for demonstration.
// /// You’d have real user/password checks here.
// pub async fn login(
//   Extension(state): Extension<Arc<RwLock<AppState>>>,
//   Json(payload): Json<LoginRequest>,
// ) -> Result<String, StatusCode> {
//   let state = state.read().await;
//   let user_opt = state.users.iter().find(|u| u.email == payload.email);

//   if let Some(user) = user_opt {
//       // Generate JWT token
//       let token = generate_jwt(&user.email, user.role.clone(), 3600); // 1h expiry
//       Ok(token)
//   } else {
//       Err(StatusCode::UNAUTHORIZED)
//   }
// }

// pub async fn proxy_response<B>(res: hyper::Response<Body>) -> AxumResponse<BoxBody> {
//   // Map the Hyper Body to Axum-compatible BoxBody
//   let (parts, body) = res.into_parts();
//   let body = boxed(AxumBody::wrap_stream(body.map_ok(Bytes::from)));

//   // Reassemble the response with the mapped body
//   AxumResponse::from_parts(parts, body)
// }

// /// Reverse proxy route: checks user role and proxies request to the correct internal service.
// pub async fn proxy_route(
//   Extension(state): Extension<Arc<RwLock<AppState>>>,
//   req: Request<axum::body::Body>,
// ) -> Result<Response, StatusCode> {
//   // Extract user claims from request extension
//   let claims = req.extensions().get::<Claims>().ok_or(StatusCode::UNAUTHORIZED)?;

//   // Access check
//   if !user_has_access(&claims.role, req.uri().path()) {
//       return Err(StatusCode::FORBIDDEN);
//   }

//   // Route to appropriate backend based on path
//   let target_uri = if req.uri().path().starts_with("/jellyfin") {
//       "http://192.168.1.10:8096"
//   } else if req.uri().path().starts_with("/files") {
//       "http://192.168.1.20"
//   } else if req.uri().path().starts_with("/admin") {
//       "http://192.168.1.30"
//   } else {
//       return Err(StatusCode::NOT_FOUND);
//   };

//   // Build a new URI by stripping the prefix and appending the rest
//   // For example, /jellyfin/xyz -> /xyz
//   let path_and_query = req
//       .uri()
//       .path_and_query()
//       .map(|pq| pq.as_str().replacen("/jellyfin", "", 1)
//                           .replacen("/files", "", 1)
//                           .replacen("/admin", "", 1))
//       .unwrap_or_else(|| "".to_string());

//   // Construct new request to internal service
//   let mut new_req = hyper::Request::builder()
//       .method(req.method())
//       .uri(format!("{}{}", target_uri, path_and_query))
//       .body(req.into_body())
//       .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//   // Copy headers from original request to new request
//   // (excluding the Host header)
//   for (k, v) in req.headers().iter() {
//       if k.as_str().to_lowercase() != "host" {
//           new_req.headers_mut().insert(k, v.clone());
//       }
//   }

//   // Use Hyper to send the request
//   let client = hyper::Client::new();
//   let res = client.request(req.map(|_| hyper::Body::empty())).await.map_err(|_| StatusCode::BAD_GATEWAY)?;

//   // Convert hyper::Response<Body> into axum::response::Response
//   // Ok(Response::from(res))
//   Ok(proxy_response::<Body>(res).await)
// }
