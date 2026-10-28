mod accounts;
mod app;
mod auth;
mod config;
mod db;
mod error;
mod health;
mod rights;
#[cfg(test)]
mod test_support;

use std::net::SocketAddr;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("miraxy_server=info")),
        )
        .init();

    let config = match config::Config::from_env() {
        Ok(config) => config,
        Err(message) => {
            tracing::error!("{message}");
            std::process::exit(1);
        }
    };

    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        let local = SocketAddr::from(([127, 0, 0, 1], config.bind.port()));
        std::process::exit(if health::probe(local).await { 0 } else { 1 });
    }

    let listener = tokio::net::TcpListener::bind(config.bind)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("impossible d'écouter sur {} : {e}", config.bind);
            std::process::exit(1);
        });
    tracing::info!("miraxy-server écoute sur {}", config.bind);
    let pool = match &config.database_url {
        Some(url) => match db::connect(url).await {
            Ok(pool) => {
                tracing::info!("base de données connectée, migrations appliquées");
                Some(pool)
            }
            Err(message) => {
                tracing::error!("{message}");
                std::process::exit(1);
            }
        },
        None => {
            tracing::warn!("MIRAXY_DATABASE_URL absente : démarrage sans base de données");
            None
        }
    };

    axum::serve(
        listener,
        app::router(&config.ui_dir, app::AppState { pool }),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("erreur du serveur");
}

/// Arrêt propre sur Ctrl+C ou SIGTERM (envoyé par podman/systemd).
async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("signal Ctrl+C") };
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("signal SIGTERM")
            .recv()
            .await;
    };
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
    tracing::info!("arrêt demandé");
}
