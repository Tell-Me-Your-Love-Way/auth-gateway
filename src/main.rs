use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;
mod config;
mod handlers;
mod models;
mod service;
#[tokio::main]
async fn main() {
    // Initialize logging system
    tracing_subscriber::fmt().init();
    

    // Load TLS certificate and private key from embedded PEM files
    let (cert, key) = config::provide_cert::gen_tls("localhost", "127.0.0.1").await;

    // Create router with single endpoint
    let router = config::provide_router::build();

    // Configure TLS settings using Rustls
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));

    // Create TCP listener with TLS encryption on port 8698
    let listener = TcpListener::new(("0.0.0.0", 443)).rustls(config.clone());

    // Create QUIC listener and combine with TCP listener
    let acceptor = QuinnListener::new(config.build_quinn_config().unwrap(), ("0.0.0.0", 443))
        .join(listener)
        .bind()
        .await;

    // Start server supporting both HTTP/3 (QUIC) and HTTPS (TCP)
    Server::new(acceptor).serve(router).await;
}
