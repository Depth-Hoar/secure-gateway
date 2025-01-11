// // Authentication (JWT) logic

// use axum::{
//   async_trait,
//   http::{Request, StatusCode},
//   middleware::{Next, from_fn, FromFnLayer},
//   response::{IntoResponse, Response},
//   body::Body,
// // //   extract::{FromRequestParts, TypedHeader},
// //   extract::{FromRequestParts, http::HeaderMap},
// };
// use http::request::Parts;
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// use once_cell::sync::Lazy;
// use serde::{Deserialize, Serialize};
// use std::time::{SystemTime, UNIX_EPOCH};
// use std::sync::Arc;
// use tower::{
//     Layer,
//     Service,
// };

// use crate::access::Role;

// // Hard-coded secret key for demonstration. DO NOT commit real secrets to Git!
// static JWT_SECRET: Lazy<String> = Lazy::new(|| "super-secret-key".to_string());

// /// JWT claims
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//   pub sub: String,     // user’s email
//   pub role: Role,      // user role
//   pub exp: usize,      // expiry
// }

// /// Minimal user representation (in-memory)
// #[derive(Debug, Clone)]
// pub struct UserToken {
//   pub email: String,
//   pub role: Role,
// }

// impl UserToken {
//   pub fn new(email: &str, role: Role) -> Self {
//       Self {
//           email: email.to_string(),
//           role,
//       }
//   }
// }

// /// Middleware that checks if request has valid JWT in the Authorization header
// pub fn jwt_auth_middleware() -> FromFnLayer<
//     fn(Request<Body>, Next<Body>) -> Result<Response, axum::http::StatusCode>,
//     (),
//     Request<Body>,
// > {
//     from_fn(authenticate_request)
// }
// /// Middleware for authenticating requests using JWT
// pub async fn authenticate_request<B>(
//     mut req: Request<B>,
//     next: Next<B>,
// ) -> Result<Response, StatusCode> {
//     // Skip authentication for the `/login` endpoint
//     if req.uri().path().starts_with("/login") {
//         return Ok(next.run(req).await);
//     }

//     // Extract the Authorization header
//     let auth_header = req.headers().get("Authorization");
//     if auth_header.is_none() {
//         return Err(StatusCode::UNAUTHORIZED);
//     }
//     let auth_value = auth_header.unwrap().to_str().unwrap_or("");

//     // Validate the format of the Authorization header (should be "Bearer <token>")
//     if !auth_value.starts_with("Bearer ") {
//         return Err(StatusCode::UNAUTHORIZED);
//     }
//     let token = &auth_value[7..]; // Strip "Bearer "

//     // Decode and validate the JWT
//     match decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
//         &Validation::default(),
//     ) {
//         Ok(token_data) => {
//             // Insert decoded claims into the request's extensions
//             req.extensions_mut().insert(token_data.claims);
//             Ok(next.run(req).await)
//         }
//         Err(_) => Err(StatusCode::UNAUTHORIZED),
//     }
// }

// /// Generate a JWT token for a user
// pub fn generate_jwt(email: &str, role: Role, expiry_in_secs: usize) -> String {
//   let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
//   let exp = now + expiry_in_secs;

//   let claims = Claims {
//       sub: email.to_string(),
//       role,
//       exp,
//   };
//   encode(
//       &Header::default(),
//       &claims,
//       &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
//   )
//   .unwrap()
// }
