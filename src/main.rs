// Entry point for the gateway

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    response::Response,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tower::{ServiceBuilder, ServiceExt};
use tokio::sync::RwLock;
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, Level};

mod auth;
mod access;
mod routes;
mod encryption;
mod logger;

use auth::{jwt_auth_middleware, Claims, UserToken};
use access::{Role, user_has_access};
use encryption::tls_server;
use logger::request_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for logging.
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Create our app state (e.g., store user list, roles, etc.)
    let app_state = Arc::new(RwLock::new(AppState::default()));

    // Build Axum router with the following route structure:
    //    /login -> Issue JWT
    //    /jellyfin -> Reverse proxy to internal Jellyfin
    //    /files -> Reverse proxy to internal file server
    //    /admin -> Reverse proxy to admin dashboard
    //
    // We attach middleware:
    //    - logging
    //    - JWT auth (except for /login)
    //    - role-based access checks
    //
    let app = Router::new()
        // Authentication route (no JWT required here)
        .route("/login", post(routes::login))
        // Protected routes
        .route("/jellyfin/*rest", get(routes::proxy_route))
        .route("/files/*rest", get(routes::proxy_route))
        .route("/admin/*rest", get(routes::proxy_route))
        // Attach shared state
        .layer(Extension(app_state))
        // Attach custom logging middleware
        .layer(tower::layer::layer_fn(request_logger))
        // Attach JWT authentication (for all routes except /login)
        .layer(jwt_auth_middleware())
        // Optionally track requests with tower-http's TraceLayer
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Start HTTPS server with Rustls
    let addr = SocketAddr::from(([0, 0, 0, 0], 8443));
    info!("Listening on https://{}", addr);

    let server = tls_server(addr, app).await?;
    server.await?;

    Ok(())
}

/// Global in-memory user database, roles, etc. 
/// In real usage, store in a DB (SQLite/Postgres) and manage properly.
#[derive(Debug)]
pub struct AppState {
    pub users: Vec<UserToken>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            users: vec![
                // Example built-in users
                UserToken::new("admin@example.com", Role::Admin),
                UserToken::new("alice@example.com", Role::Family),
                UserToken::new("bob@example.com", Role::Guest),
            ],
        }
    }
}

