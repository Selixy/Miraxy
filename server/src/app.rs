//! Assemblage des routes de l'API.

use std::path::Path;

use axum::{
    Router,
    http::{HeaderValue, header},
    routing::get,
};
use sqlx::PgPool;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

use crate::{accounts, error::ApiError, health::health};

/// État partagé des routes. Sans `MIRAXY_DATABASE_URL`, `pool` est vide et les routes qui en ont besoin
/// répondent 503.
#[derive(Clone, Default)]
pub struct AppState {
    pub pool: Option<PgPool>,
}

impl AppState {
    pub fn pool(&self) -> Result<&PgPool, ApiError> {
        self.pool.as_ref().ok_or(ApiError::Unavailable)
    }
}

/// L'API sous `/api`, tout le reste vient des fichiers statiques de l'ui.
/// `no-cache` : le navigateur et le CDN revalident à chaque fois (ETag/Last-Modified), pour qu'un
/// déploiement soit visible tout de suite.
pub fn router(ui_dir: &Path, state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .merge(accounts::routes())
        .with_state(state)
        .fallback_service(ServeDir::new(ui_dir))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_ok() {
        let response = router(Path::new("../ui"), AppState::default())
            .oneshot(Request::get("/api/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["ok"], true);
        assert_eq!(json["protocol"], miraxy_common::PROTOCOL_VERSION);
    }

    #[tokio::test]
    async fn home_page_is_served() {
        let response = router(Path::new("../ui"), AppState::default())
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn static_files_are_revalidated() {
        let response = router(Path::new("../ui"), AppState::default())
            .oneshot(Request::get("/css/main.css").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.headers()["cache-control"], "no-cache");
    }

    #[tokio::test]
    async fn login_page_is_served() {
        let response = router(Path::new("../ui"), AppState::default())
            .oneshot(Request::get("/login/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn signup_page_is_served() {
        let response = router(Path::new("../ui"), AppState::default())
            .oneshot(Request::get("/signup/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn unknown_route_is_404() {
        let response = router(Path::new("../ui"), AppState::default())
            .oneshot(Request::get("/api/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
