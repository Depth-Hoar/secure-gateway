// use std::{fs::File, io::BufReader};
// use rustls::{Certificate, PrivateKey, ServerConfig};
// use rustls_pemfile::{certs, pkcs8_private_keys};

// pub fn load_rustls_config() -> ServerConfig {
//     let cert_file = File::open("certs/cert.pem").expect("certificate file not found");
//     let mut cert_reader = BufReader::new(cert_file);
//     let cert_chain = certs(&mut cert_reader)
//         .expect("invalid cert")
//         .into_iter()
//         .map(Certificate)
//         .collect::<Vec<_>>();

//     let key_file = File::open("certs/key.pem").expect("key file not found");
//     let mut key_reader = BufReader::new(key_file);
//     let mut keys = pkcs8_private_keys(&mut key_reader).expect("invalid key");
//     let private_key = PrivateKey(keys.remove(0));

//     ServerConfig::builder()
//         .with_safe_defaults()
//         .with_no_client_auth()
//         .with_single_cert(cert_chain, private_key)
//         .expect("bad certificates/private key")
// }
