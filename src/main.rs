// src/main.rs
use axum_server::Server;
use hyper::{Body, Request, Response};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tokio::main;
use tower::Service;

// A trivial "fake" Tower service that meets the `Sync + Send + Clone` constraints.
#[derive(Clone)]
struct FakeService;

impl Service<Request<Body>> for FakeService {
    type Response = Response<Body>;
    type Error = Infallible;
    // You can also use `Pin<Box<dyn Future<...>>>`; 
    // but here we do a simple ready-based approach.
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Always ready to serve
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        // Return a simple "Hello" response
        let response = Response::new(Body::from("Hello from FakeService!"));
        std::future::ready(Ok(response))
    }
}

#[main]
async fn main() {
    // A trivial service that definitely compiles with `axum_server`
    let service = FakeService;

    // Start an HTTPS server on port 8443
    Server::new()
        .bind_rustls("0.0.0.0:8443")
        .private_key_file("certs/key.pem")
        .certificate_file("certs/cert.pem")
        // NOTE: `.serve(...)` requires your service to be `Send + Sync + Clone + 'static`
        .serve(service)
        .await
        .unwrap();
}























// use axum::{
//     extract::Extension,
//     routing::{get, post},
//     Router,
// };
// use axum_server::{
//     bind_rustls,
//     tls::rustls::RustlsConfig, // <-- Correct import for RustlsConfig
// };
// use tower::{ServiceBuilder};
// use tower_http::trace::TraceLayer;
// use tokio::sync::RwLock;
// use tracing::{info, Level};
// use std::{sync::Arc, error::Error};

// mod auth;
// mod access;
// mod routes;
// mod logger;

// use auth::{jwt_auth_middleware, UserToken};
// use access::Role;
// use logger::request_logger;

// /// Global in-memory user database, roles, etc.
// /// In a real-world application, replace this with a proper database.
// #[derive(Debug)]
// pub struct AppState {
//     pub users: Vec<UserToken>,
// }

// impl Default for AppState {
//     fn default() -> Self {
//         Self {
//             users: vec![
//                 UserToken::new("admin@example.com", Role::Admin),
//                 UserToken::new("alice@example.com", Role::Family),
//                 UserToken::new("bob@example.com", Role::Guest),
//             ],
//         }
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Initialize tracing subscriber for logging.
//     tracing_subscriber::fmt()
//         .with_max_level(Level::INFO)
//         .init();

//     // Load Rustls certificates (enable the "tls-rustls" feature in Cargo.toml)
//     let tls_config = RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem")
//         .await
//         .expect("Failed to load certificates");

//     // Create our shared app state
//     let app_state = Arc::new(RwLock::new(AppState::default()));

//     // Build Axum router
//     let app = Router::new()
//         // Public route
//         .route("/", get(|| async { "Hello HTTPS World!" }))
//         // Authentication route (JWT not required here)
//         .route("/login", post(routes::login))
//         // Protected routes
//         .route("/jellyfin/*rest", get(routes::proxy_route))
//         .route("/files/*rest", get(routes::proxy_route))
//         .route("/admin/*rest", get(routes::proxy_route))
//         // Middleware layers
//         .layer(Extension(app_state))
//         .layer(tower::layer::layer_fn(request_logger))
//         .layer(jwt_auth_middleware())
//         .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

//     // Start the HTTPS server on port 8443
//     bind_rustls("0.0.0.0:8443", tls_config)
//         .serve(app.into_make_service())
//         .await?;

//     Ok(())
// }
