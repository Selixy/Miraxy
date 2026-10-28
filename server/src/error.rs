//! Erreurs de l'API : un code stable pour le code client, un message pour l'humain.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    /// Champ manquant ou invalide.
    Invalid(&'static str),
    EmailTaken,
    InvalidCredentials,
    Unauthorized,
    /// Pas de base de données configurée.
    Unavailable,
    Internal,
}

impl ApiError {
    pub fn internal(error: impl std::fmt::Display) -> Self {
        tracing::error!("erreur interne : {error}");
        ApiError::Internal
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::Invalid(message) => (StatusCode::BAD_REQUEST, "invalid", message),
            ApiError::EmailTaken => (
                StatusCode::CONFLICT,
                "email_taken",
                "Cette adresse e-mail est déjà utilisée.",
            ),
            ApiError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                "Adresse e-mail ou mot de passe incorrect.",
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Connexion requise.",
            ),
            ApiError::Unavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "unavailable",
                "Service momentanément indisponible.",
            ),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
                "Erreur interne.",
            ),
        };
        (status, Json(json!({ "error": code, "message": message }))).into_response()
    }
}
