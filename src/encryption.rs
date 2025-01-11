// // TLS/SSL setup helpers

// use axum::{Router};
// use hyper::server::Server;
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tokio_rustls::{
//     rustls::{Certificate, PrivateKey, ServerConfig},
//     TlsAcceptor,
// };
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
// use anyhow::Result;

// /// Configure Rustls, load certificates, and launch the TLS server.
// pub async fn tls_server(addr: SocketAddr, router: Router) -> Result<hyper::server::Server<TlsAcceptor, axum::body::Body>> {
//     let (certs, key) = load_certs_and_key().await?;
//     let server_config = build_server_config(certs, key)?;

//     let acceptor = TlsAcceptor::from(Arc::new(server_config));
//     let server = Server::bind(&addr).serve(tower::make::Shared::new(
//         hyper::service::make_service_fn(move |_| {
//             let acceptor = acceptor.clone();
//             let service = router.clone();
//             async move {
//                 Ok::<_, hyper::Error>(hyper::service::service_fn(move |conn| {
//                     let acceptor = acceptor.clone();
//                     let service = service.clone();
//                     async move {
//                         let tls_stream = acceptor.accept(conn).await?;
//                         hyper::service::service_fn(move |req| {
//                             service.clone().oneshot(req)
//                         })(tls_stream).await
//                     }
//                 }))
//             }
//         }),
//     ));

//     Ok(server)
// }

// /// Loads certificate and private key from local files.
// /// Replace with your own file paths or integrate with Let’s Encrypt in production.
// async fn load_certs_and_key() -> Result<(Vec<Certificate>, PrivateKey)> {
//     // For demonstration, use self-signed cert and key:
//     let mut cert_file = File::open("certs/cert.pem").await?;
//     let mut cert_data = vec![];
//     cert_file.read_to_end(&mut cert_data).await?;

//     let mut key_file = File::open("certs/key.pem").await?;
//     let mut key_data = vec![];
//     key_file.read_to_end(&mut key_data).await?;

//     let certs = rustls_pemfile::certs(&mut &cert_data[..])?
//         .into_iter()
//         .map(rustls::Certificate)
//         .collect();

//     let mut keys = rustls_pemfile::pkcs8_private_keys(&mut &key_data[..])?;
//     if keys.is_empty() {
//         anyhow::bail!("No valid private keys found.");
//     }

//     let key = rustls::PrivateKey(keys.remove(0));
//     Ok((certs, key))
// }

// /// Build a basic ServerConfig for TLS
// fn build_server_config(certs: Vec<Certificate>, key: PrivateKey) -> Result<ServerConfig> {
//     let mut cfg = rustls::ServerConfig::builder()
//         .with_safe_defaults()
//         .with_no_client_auth()
//         .with_single_cert(certs, key)?;
//     // Configure TLS settings if needed (e.g., cipher suites, ALPN, etc.)
//     Ok(cfg)
// }
