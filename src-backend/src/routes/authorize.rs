use argon2::{
    password_hash::{Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    Json,
};
use axum_extra::extract::cookie::Cookie;
use chrono::Duration;
use sqlx::prelude::FromRow;

use crate::AppState;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Claims {
    account_id: i32,
    exp: usize,
}

#[derive(FromRow, Debug, Deserialize)]
struct Account {
    id: i32,
    password: String,
}

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Response<Body> {
    let query = "Insert into account (email, password) values ($1, $2)";
    sqlx::query(query)
        .bind(&credentials.email)
        .bind(hash_password(&credentials.password, &state.salt))
        .fetch_optional(&state.pool)
        .await
        .map(|_| Response::new(Body::empty()))
        .unwrap_or_else(|_| {
            // todo map this to a proper response
            Response::builder()
                .status(StatusCode::CONFLICT)
                .body(Body::empty())
                .unwrap()
        })
}

pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Response<Body> {
    let query = "Select id, password from account where email = $1";

    let account = sqlx::query_as::<_, Account>(query)
        .bind(&credentials.email)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    // map account to a response, if acc is empty return unauthorized
    generate_token(account, &credentials)
}

fn generate_token(account: Option<Account>, credentials: &Credentials) -> Response<Body> {
    account
        .map(|acc| {
            if verify_password_hash(&credentials.password, &acc.password).is_err() {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap();
            }

            let my_claims = Claims {
                account_id: acc.id,

                exp: (chrono::Utc::now() + Duration::days(1)).timestamp() as usize,
            };

            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .unwrap();

            let mut cookie = Cookie::new("jwt", token);

            cookie.set_path("/");

            Response::builder()
                .header("Set-Cookie", cookie.to_string())
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap()
        })
        .unwrap_or_else(|| {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap()
        })
}

pub fn verify_password_hash(
    password_candidate: &str,
    expected_password_string: &str,
) -> Result<(), Error> {
    let argon2 = Argon2::default();
    let expected_password_hash = PasswordHash::new(expected_password_string).unwrap();

    argon2.verify_password(password_candidate.as_bytes(), &expected_password_hash)
}

pub fn hash_password(password: &str, salt: &SaltString) -> String {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), salt).unwrap();

    password_hash.to_string()
}
