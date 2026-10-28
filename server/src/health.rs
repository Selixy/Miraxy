//! Route de santé : sert aux sondes de la réception et à la supervision.

use std::{net::SocketAddr, time::Duration};

use axum::Json;
use serde::Serialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};

#[derive(Serialize)]
pub struct Health {
    ok: bool,
    version: &'static str,
    protocol: u32,
}

pub async fn health() -> Json<Health> {
    Json(Health {
        ok: true,
        version: env!("CARGO_PKG_VERSION"),
        protocol: miraxy_common::PROTOCOL_VERSION,
    })
}

/// Interroge `/api/health` d'un serveur en marche. Sert à la commande `healthcheck` :
/// l'image n'a ni curl ni wget, le binaire se sonde lui-même.
pub async fn probe(addr: SocketAddr) -> bool {
    let request = format!("GET /api/health HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    let attempt = async {
        let mut stream = TcpStream::connect(addr).await.ok()?;
        stream.write_all(request.as_bytes()).await.ok()?;
        let mut response = String::new();
        stream.read_to_string(&mut response).await.ok()?;
        Some(response.starts_with("HTTP/1.1 200"))
    };
    matches!(
        timeout(Duration::from_secs(3), attempt).await,
        Ok(Some(true))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn probe_succeeds_on_running_server() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(
                listener,
                crate::app::router(std::path::Path::new("../ui"), Default::default()),
            )
            .await
            .unwrap()
        });
        assert!(probe(addr).await);
    }

    #[tokio::test]
    async fn probe_fails_when_nothing_listens() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);
        assert!(!probe(addr).await);
    }
}
