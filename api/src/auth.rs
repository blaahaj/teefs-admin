use actix_web::{
    HttpResponse, Responder,
    cookie::{CookieBuilder, SameSite},
    http::header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS,
    },
    options, post, web,
};
use iso8601_timestamp::time::SignedDuration;
use serde::{Deserialize, Serialize};

use crate::AppData;

#[derive(Debug, Clone, Deserialize)]
struct AuthLoginRequest {
    secret: String,
}

#[derive(Debug, Clone, Serialize)]
struct AuthLoginResponse {
    ok: bool,
}

#[post("/admin/api/auth/login")]
async fn auth_login_handler(
    payload: web::Json<AuthLoginRequest>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    if payload.secret.eq(&app_data.correct_secret) {
        HttpResponse::Ok()
            .cookie(
                CookieBuilder::new("auth", &payload.secret)
                    .same_site(SameSite::Strict)
                    .path("/admin/api")
                    .http_only(true)
                    .max_age(SignedDuration::seconds(1800))
                    .finish(),
            )
            .json(AuthLoginResponse { ok: true })
    } else {
        HttpResponse::Ok()
            .cookie(
                CookieBuilder::new("auth", "...")
                    .same_site(SameSite::Strict)
                    .path("/admin/api")
                    .http_only(true)
                    .max_age(SignedDuration::seconds(1))
                    .finish(),
            )
            .json(AuthLoginResponse { ok: false })
    }
}

#[post("/admin/api/auth/logout")]
pub async fn auth_logout_handler() -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            CookieBuilder::new("auth", "...")
                .same_site(SameSite::Strict)
                .path("/admin/api")
                .http_only(true)
                .max_age(SignedDuration::seconds(1))
                .finish(),
        )
        .json(AuthLoginResponse { ok: false })
}

#[options("/admin/api/auth/{q}*")]
pub async fn cors_preflight_handler() -> impl Responder {
    HttpResponse::Ok()
        .append_header((ACCESS_CONTROL_ALLOW_METHODS, "POST,OPTIONS"))
        .append_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .append_header((ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type"))
        .finish()
}
