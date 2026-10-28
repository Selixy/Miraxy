//! Aide aux tests qui ont besoin d'une vraie base PostgreSQL.
//! Ils ne tournent que si `MIRAXY_TEST_DATABASE_URL` est définie (une base d'où l'on peut créer
//! d'autres bases) ; sinon ils sont ignorés, comme dans le build de l'image, qui n'a pas de base.

use sqlx::{
    Connection, PgConnection,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;

use crate::{app::AppState, db};

/// Une base neuve et migrée, propre au test. `None` si aucune base de test n'est configurée.
pub async fn test_state() -> Option<AppState> {
    let url = std::env::var("MIRAXY_TEST_DATABASE_URL").ok()?;
    let options = PgConnectOptions::from_str(&url).expect("MIRAXY_TEST_DATABASE_URL invalide");
    let name = format!("test_{}", uuid::Uuid::new_v4().simple());

    let mut admin = PgConnection::connect_with(&options).await.unwrap();
    // Le nom vient d'un uuid généré ici : pas d'injection possible.
    sqlx::query(sqlx::AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options.database(&name))
        .await
        .unwrap();
    db::migrate(&pool).await.unwrap();
    Some(AppState { pool: Some(pool) })
}
