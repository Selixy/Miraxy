//! Inscription, connexion, déconnexion et « qui suis-je ».

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::AppState,
    auth::{self, CurrentAccount, PASSWORD_MAX_BYTES, PASSWORD_MIN_CHARS},
    error::ApiError,
    rights::Right,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/signup", post(signup))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/me", get(me))
}

#[derive(Deserialize)]
struct SignupBody {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginBody {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct DepartmentRights {
    department: String,
    rights: Vec<String>,
}

#[derive(Serialize)]
struct Me {
    id: Uuid,
    email: String,
    name: String,
    /// Tous les droits dans tous les départements : `departments` ne le détaille alors pas.
    is_admin: bool,
    departments: Vec<DepartmentRights>,
}

fn normalize_email(raw: &str) -> Result<String, ApiError> {
    let email = raw.trim().to_lowercase();
    let valid = email.len() <= 254
        && email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty() && domain.contains('.') && !domain.ends_with('.')
        })
        && !email.contains(char::is_whitespace);
    if valid {
        Ok(email)
    } else {
        Err(ApiError::Invalid("Adresse e-mail invalide."))
    }
}

fn check_password(password: &str) -> Result<(), ApiError> {
    if password.chars().count() < PASSWORD_MIN_CHARS {
        return Err(ApiError::Invalid(
            "Le mot de passe doit faire au moins 10 caractères.",
        ));
    }
    if password.len() > PASSWORD_MAX_BYTES {
        return Err(ApiError::Invalid("Mot de passe trop long."));
    }
    Ok(())
}

async fn load_me(pool: &PgPool, account: CurrentAccount) -> Result<Me, ApiError> {
    let rows: Vec<(String, String)> = sqlx::query_as(
        "select d.slug, r.right_name
         from account_rights r join departments d on d.id = r.department_id
         where r.account_id = $1
         order by d.slug, r.right_name",
    )
    .bind(account.id)
    .fetch_all(pool)
    .await
    .map_err(ApiError::internal)?;

    let mut departments: Vec<DepartmentRights> = Vec::new();
    for (slug, name) in rows {
        // Un droit inconnu du catalogue (retiré du code) ne doit pas être annoncé.
        if Right::parse(&name).is_none() {
            continue;
        }
        match departments.last_mut() {
            Some(last) if last.department == slug => last.rights.push(name),
            _ => departments.push(DepartmentRights {
                department: slug,
                rights: vec![name],
            }),
        }
    }
    Ok(Me {
        id: account.id,
        email: account.email,
        name: account.name,
        is_admin: account.is_admin,
        departments,
    })
}

async fn signup(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<SignupBody>,
) -> Result<(StatusCode, CookieJar, Json<Me>), ApiError> {
    let pool = state.pool()?;
    let name = body.name.trim().to_string();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(ApiError::Invalid(
            "Le nom doit faire entre 1 et 80 caractères.",
        ));
    }
    let email = normalize_email(&body.email)?;
    check_password(&body.password)?;
    let password_hash = auth::hash_password(body.password).await?;

    // Le premier compte créé devient administrateur (amorçage). Le verrou évite que deux
    // inscriptions simultanées le soient toutes les deux.
    let mut tx = pool.begin().await.map_err(ApiError::internal)?;
    sqlx::query("select pg_advisory_xact_lock(7101)")
        .execute(&mut *tx)
        .await
        .map_err(ApiError::internal)?;
    let inserted = sqlx::query_as::<_, CurrentAccount>(
        "insert into accounts (email, name, password_hash, is_admin)
         values ($1, $2, $3, not exists (select 1 from accounts))
         returning id, email, name, is_admin",
    )
    .bind(&email)
    .bind(&name)
    .bind(&password_hash)
    .fetch_one(&mut *tx)
    .await;
    let account = match inserted {
        Ok(account) => account,
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            return Err(ApiError::EmailTaken);
        }
        Err(e) => return Err(ApiError::internal(e)),
    };
    tx.commit().await.map_err(ApiError::internal)?;

    let jar = auth::start_session(pool, jar, account.id).await?;
    Ok((
        StatusCode::CREATED,
        jar,
        Json(load_me(pool, account).await?),
    ))
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginBody>,
) -> Result<(CookieJar, Json<Me>), ApiError> {
    let pool = state.pool()?;
    if body.password.len() > PASSWORD_MAX_BYTES {
        return Err(ApiError::InvalidCredentials);
    }
    let email = body.email.trim().to_lowercase();
    let found: Option<(Uuid, String, String, bool, String)> = sqlx::query_as(
        "select id, email, name, is_admin, password_hash from accounts where email = $1",
    )
    .bind(&email)
    .fetch_optional(pool)
    .await
    .map_err(ApiError::internal)?;

    let Some((id, email, name, is_admin, hash)) = found else {
        // Même coût que pour un compte existant.
        auth::verify_password(body.password, auth::dummy_hash().await).await;
        return Err(ApiError::InvalidCredentials);
    };
    if !auth::verify_password(body.password, hash).await {
        return Err(ApiError::InvalidCredentials);
    }
    let account = CurrentAccount {
        id,
        email,
        name,
        is_admin,
    };
    let jar = auth::start_session(pool, jar, id).await?;
    Ok((jar, Json(load_me(pool, account).await?)))
}

async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar), ApiError> {
    let jar = auth::end_session(state.pool()?, jar).await?;
    Ok((StatusCode::NO_CONTENT, jar))
}

async fn me(State(state): State<AppState>, account: CurrentAccount) -> Result<Json<Me>, ApiError> {
    Ok(Json(load_me(state.pool()?, account).await?))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use axum::{
        body::Body,
        http::{Request, Response, header},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use super::*;
    use crate::{app::router, test_support::test_state};

    const PASSWORD: &str = "un mot de passe solide";

    async fn call(
        state: &AppState,
        method: &str,
        uri: &str,
        cookie: Option<&str>,
        body: Option<Value>,
    ) -> Response<Body> {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        let body = match body {
            Some(json) => {
                request = request.header(header::CONTENT_TYPE, "application/json");
                Body::from(json.to_string())
            }
            None => Body::empty(),
        };
        router(Path::new("../ui"), state.clone())
            .oneshot(request.body(body).unwrap())
            .await
            .unwrap()
    }

    async fn json_of(response: Response<Body>) -> Value {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    }

    /// `miraxy_session=...` à renvoyer, lu dans le Set-Cookie d'une réponse.
    fn session_cookie(response: &Response<Body>) -> String {
        let header = response.headers()[header::SET_COOKIE].to_str().unwrap();
        header.split(';').next().unwrap().to_string()
    }

    async fn signup(state: &AppState, name: &str, email: &str) -> Response<Body> {
        call(
            state,
            "POST",
            "/api/signup",
            None,
            Some(json!({ "name": name, "email": email, "password": PASSWORD })),
        )
        .await
    }

    #[tokio::test]
    async fn without_database_the_routes_say_unavailable() {
        let response = call(&AppState::default(), "GET", "/api/me", None, None).await;
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn first_account_is_admin_the_next_is_not() {
        let Some(state) = test_state().await else {
            return;
        };
        let first = signup(&state, "Alice", "alice@example.org").await;
        assert_eq!(first.status(), StatusCode::CREATED);
        assert_eq!(json_of(first).await["is_admin"], true);

        let second = signup(&state, "Bob", "bob@example.org").await;
        assert_eq!(second.status(), StatusCode::CREATED);
        assert_eq!(json_of(second).await["is_admin"], false);
    }

    #[tokio::test]
    async fn cookie_is_hardened_and_the_password_is_not_stored_in_clear() {
        let Some(state) = test_state().await else {
            return;
        };
        let response = signup(&state, "Alice", "alice@example.org").await;
        let set_cookie = response.headers()[header::SET_COOKIE]
            .to_str()
            .unwrap()
            .to_string();
        assert!(set_cookie.contains("HttpOnly") && set_cookie.contains("Secure"));
        assert!(set_cookie.contains("SameSite=Lax"));

        let stored: String = sqlx::query_scalar("select password_hash from accounts")
            .fetch_one(state.pool().unwrap())
            .await
            .unwrap();
        assert!(stored.starts_with("$argon2id$") && !stored.contains(PASSWORD));
    }

    #[tokio::test]
    async fn email_is_unique_whatever_the_case() {
        let Some(state) = test_state().await else {
            return;
        };
        signup(&state, "Alice", "alice@example.org").await;
        let again = signup(&state, "Autre", "  ALICE@Example.org ").await;
        assert_eq!(again.status(), StatusCode::CONFLICT);
        assert_eq!(json_of(again).await["error"], "email_taken");
    }

    #[tokio::test]
    async fn invalid_input_is_refused() {
        let Some(state) = test_state().await else {
            return;
        };
        let bad_email = signup(&state, "Alice", "pas-un-email").await;
        assert_eq!(bad_email.status(), StatusCode::BAD_REQUEST);
        let short = call(
            &state,
            "POST",
            "/api/signup",
            None,
            Some(json!({ "name": "A", "email": "a@example.org", "password": "court" })),
        )
        .await;
        assert_eq!(short.status(), StatusCode::BAD_REQUEST);
        let no_name = signup(&state, "   ", "b@example.org").await;
        assert_eq!(no_name.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn login_then_me_then_logout() {
        let Some(state) = test_state().await else {
            return;
        };
        signup(&state, "Alice", "alice@example.org").await;

        let login = call(
            &state,
            "POST",
            "/api/login",
            None,
            Some(json!({ "email": "Alice@example.org", "password": PASSWORD })),
        )
        .await;
        assert_eq!(login.status(), StatusCode::OK);
        let cookie = session_cookie(&login);

        let me = call(&state, "GET", "/api/me", Some(&cookie), None).await;
        assert_eq!(me.status(), StatusCode::OK);
        assert_eq!(json_of(me).await["email"], "alice@example.org");

        let out = call(&state, "POST", "/api/logout", Some(&cookie), None).await;
        assert_eq!(out.status(), StatusCode::NO_CONTENT);
        let after = call(&state, "GET", "/api/me", Some(&cookie), None).await;
        assert_eq!(after.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn wrong_password_and_unknown_email_look_the_same() {
        let Some(state) = test_state().await else {
            return;
        };
        signup(&state, "Alice", "alice@example.org").await;
        let wrong = call(
            &state,
            "POST",
            "/api/login",
            None,
            Some(json!({ "email": "alice@example.org", "password": "mauvais mot de passe" })),
        )
        .await;
        let unknown = call(
            &state,
            "POST",
            "/api/login",
            None,
            Some(json!({ "email": "inconnu@example.org", "password": PASSWORD })),
        )
        .await;
        assert_eq!(wrong.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(unknown.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(json_of(wrong).await, json_of(unknown).await);
    }

    #[tokio::test]
    async fn me_requires_a_valid_session() {
        let Some(state) = test_state().await else {
            return;
        };
        let none = call(&state, "GET", "/api/me", None, None).await;
        assert_eq!(none.status(), StatusCode::UNAUTHORIZED);
        let forged = call(&state, "GET", "/api/me", Some("miraxy_session=abc"), None).await;
        assert_eq!(forged.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn expired_session_is_refused() {
        let Some(state) = test_state().await else {
            return;
        };
        let response = signup(&state, "Alice", "alice@example.org").await;
        let cookie = session_cookie(&response);
        sqlx::query("update sessions set expires_at = now() - interval '1 second'")
            .execute(state.pool().unwrap())
            .await
            .unwrap();
        let me = call(&state, "GET", "/api/me", Some(&cookie), None).await;
        assert_eq!(me.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rights_are_listed_per_department() {
        let Some(state) = test_state().await else {
            return;
        };
        let pool = state.pool().unwrap();
        signup(&state, "Alice", "alice@example.org").await;
        let bob = signup(&state, "Bob", "bob@example.org").await;
        let cookie = session_cookie(&bob);

        let fx: Uuid = sqlx::query_scalar(
            "insert into departments (slug, name) values ('fx', 'FX') returning id",
        )
        .fetch_one(pool)
        .await
        .unwrap();
        let modeling: Uuid = sqlx::query_scalar(
            "insert into departments (slug, name) values ('modeling', 'Modélisation') returning id",
        )
        .fetch_one(pool)
        .await
        .unwrap();
        let bob_id: Uuid =
            sqlx::query_scalar("select id from accounts where email = 'bob@example.org'")
                .fetch_one(pool)
                .await
                .unwrap();
        for (department, right) in [
            (fx, Right::PublishTasks),
            (fx, Right::PublishResults),
            (modeling, Right::PublishResults),
        ] {
            sqlx::query("insert into account_rights values ($1, $2, $3)")
                .bind(bob_id)
                .bind(department)
                .bind(right.as_str())
                .execute(pool)
                .await
                .unwrap();
        }
        // Un droit retiré du catalogue ne doit pas être annoncé.
        sqlx::query("insert into account_rights values ($1, $2, 'droit_disparu')")
            .bind(bob_id)
            .bind(modeling)
            .execute(pool)
            .await
            .unwrap();

        let me = json_of(call(&state, "GET", "/api/me", Some(&cookie), None).await).await;
        assert_eq!(me["is_admin"], false);
        assert_eq!(
            me["departments"],
            json!([
                { "department": "fx", "rights": ["publish_results", "publish_tasks"] },
                { "department": "modeling", "rights": ["publish_results"] },
            ])
        );
    }

    #[tokio::test]
    async fn deleting_an_account_removes_its_sessions_and_rights() {
        let Some(state) = test_state().await else {
            return;
        };
        let pool = state.pool().unwrap();
        signup(&state, "Alice", "alice@example.org").await;
        sqlx::query("delete from accounts")
            .execute(pool)
            .await
            .unwrap();
        let sessions: i64 = sqlx::query_scalar("select count(*) from sessions")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(sessions, 0);
    }
}
