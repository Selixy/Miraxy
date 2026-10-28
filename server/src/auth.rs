//! Mots de passe (Argon2id) et sessions (jeton aléatoire dans un cookie, empreinte en base).

use std::sync::OnceLock;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::AppState, error::ApiError};

pub const SESSION_COOKIE: &str = "miraxy_session";
const SESSION_DAYS: i64 = 30;

pub const PASSWORD_MIN_CHARS: usize = 10;
/// Plafond : Argon2 coûte cher, on ne hache pas des mots de passe de plusieurs Mo.
pub const PASSWORD_MAX_BYTES: usize = 1024;

/// Hachage hors du thread asynchrone : Argon2 est volontairement lent.
pub async fn hash_password(password: String) -> Result<String, ApiError> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(ApiError::internal)
    })
    .await
    .map_err(ApiError::internal)?
}

pub async fn verify_password(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .map(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            })
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false)
}

/// Empreinte à comparer quand l'adresse est inconnue : la réponse prend le même temps que pour
/// un vrai compte, on ne révèle pas quelles adresses existent.
pub async fn dummy_hash() -> String {
    static DUMMY: OnceLock<String> = OnceLock::new();
    if let Some(hash) = DUMMY.get() {
        return hash.clone();
    }
    let hash = hash_password("mot de passe factice".into())
        .await
        .unwrap_or_default();
    DUMMY.get_or_init(|| hash).clone()
}

fn token_hash(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

fn new_token() -> Result<String, ApiError> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(ApiError::internal)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Ouvre une session : ajoute l'empreinte en base et le cookie à la réponse.
pub async fn start_session(
    pool: &PgPool,
    jar: CookieJar,
    account_id: Uuid,
) -> Result<CookieJar, ApiError> {
    let token = new_token()?;
    sqlx::query(
        "insert into sessions (token_hash, account_id, expires_at)
         values ($1, $2, now() + make_interval(days => $3::int))",
    )
    .bind(token_hash(&token))
    .bind(account_id)
    .bind(SESSION_DAYS as i32)
    .execute(pool)
    .await
    .map_err(ApiError::internal)?;

    let cookie = Cookie::build((SESSION_COOKIE, token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(SESSION_DAYS))
        .build();
    Ok(jar.add(cookie))
}

/// Ferme la session du cookie (si elle existe) et efface le cookie.
pub async fn end_session(pool: &PgPool, jar: CookieJar) -> Result<CookieJar, ApiError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        sqlx::query("delete from sessions where token_hash = $1")
            .bind(token_hash(cookie.value()))
            .execute(pool)
            .await
            .map_err(ApiError::internal)?;
    }
    Ok(jar.remove(Cookie::build(SESSION_COOKIE).path("/").build()))
}

/// Le compte de la session en cours. Sert d'extracteur : une route qui le demande exige une connexion.
#[derive(sqlx::FromRow)]
pub struct CurrentAccount {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub is_admin: bool,
}

impl FromRequestParts<AppState> for CurrentAccount {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let pool = state.pool()?;
        let jar = CookieJar::from_headers(&parts.headers);
        let cookie = jar.get(SESSION_COOKIE).ok_or(ApiError::Unauthorized)?;
        sqlx::query_as::<_, CurrentAccount>(
            "select a.id, a.email, a.name, a.is_admin
             from sessions s join accounts a on a.id = s.account_id
             where s.token_hash = $1 and s.expires_at > now()",
        )
        .bind(token_hash(cookie.value()))
        .fetch_optional(pool)
        .await
        .map_err(ApiError::internal)?
        .ok_or(ApiError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn password_round_trip() {
        let hash = hash_password("un mot de passe long".into()).await.unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_password("un mot de passe long".into(), hash.clone()).await);
        assert!(!verify_password("autre chose".into(), hash).await);
    }

    #[test]
    fn tokens_are_long_and_distinct() {
        let (a, b) = (new_token().unwrap(), new_token().unwrap());
        assert_eq!(a.len(), 64);
        assert_ne!(a, b);
    }
}
