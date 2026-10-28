//! Connexion à PostgreSQL et migrations.

use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

/// La base peut démarrer en même temps que le serveur : on réessaie pendant 30 s.
pub async fn connect(url: &str) -> Result<PgPool, String> {
    let mut last_error = String::new();
    for attempt in 1..=15 {
        match PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(url)
            .await
        {
            Ok(pool) => {
                migrate(&pool).await?;
                return Ok(pool);
            }
            Err(e) => {
                last_error = e.to_string();
                tracing::warn!("base de données injoignable (essai {attempt}/15) : {last_error}");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
    Err(format!("connexion à la base impossible : {last_error}"))
}

pub async fn migrate(pool: &PgPool) -> Result<(), String> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| format!("migrations en échec : {e}"))
}
