mod auth;
mod db;
mod random_local_parts;
mod types;

use actix_web::middleware::Logger;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, options, post, web};
use env_logger::Env;
use rusqlite::{self, Error::SqliteFailure, ErrorCode::ConstraintViolation};
use serde::Serialize;

use std::env;
use std::env::VarError::{NotPresent, NotUnicode};

use crate::auth::{auth_login_handler, auth_logout_handler};
use crate::types::{AddMailboxRequest, MailboxResponse, MailboxesResponse};

#[derive(Debug)]
pub struct AppData {
    pub db_file: String,
    pub correct_secret: String,
}

#[derive(Serialize)]
struct SimpleMessageResponse {
    message: &'static str,
}

#[get("/")]
async fn slash_handler() -> impl Responder {
    HttpResponse::Ok().json(SimpleMessageResponse {
        message: "Hello, world!",
    })
}

#[get("/ping")]
async fn ping_handler() -> impl Responder {
    HttpResponse::Ok().json(SimpleMessageResponse { message: "pong" })
}

#[options("/admin/api/{tail}*")]
async fn preflight_handler() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/admin/api/mailboxes")]
async fn list_mailboxes_handler(req: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    if req.cookie("auth").map(|x| x.value().to_owned()) != Some(app_data.correct_secret.clone()) {
        return HttpResponse::Forbidden().finish();
    }

    match db::list_mailboxes(&app_data.db_file) {
        Ok(mailboxes) => HttpResponse::Ok().json(MailboxesResponse { mailboxes }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/admin/api/mailboxes/{id}")]
async fn get_mailbox_handler(
    req: HttpRequest,
    info: web::Path<String>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    if req.cookie("auth").map(|x| x.value().to_owned()) != Some(app_data.correct_secret.clone()) {
        return HttpResponse::Forbidden().finish();
    }

    let id = info.into_inner();

    match db::get_mailbox(&app_data.db_file, &id) {
        Ok(mailbox) => match mailbox {
            Some(m) => HttpResponse::Ok().json(MailboxResponse { mailbox: m }),
            None => HttpResponse::NotFound().body(""),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/admin/api/mailboxes/{id}/delete")]
async fn delete_mailbox_handler(
    req: HttpRequest,
    info: web::Path<String>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    if req.cookie("auth").map(|x| x.value().to_owned()) != Some(app_data.correct_secret.clone()) {
        return HttpResponse::Forbidden().finish();
    }

    let id = info.into_inner();

    let r = db::delete_mailbox(&app_data.db_file, &id);

    match r {
        Ok(opt_mbx) => match opt_mbx {
            Some(mbx) => HttpResponse::Ok().json(MailboxResponse { mailbox: mbx }),
            None => HttpResponse::NotFound().body(""),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/admin/api/mailboxes/{id}/undelete")]
async fn undelete_mailbox_handler(
    req: HttpRequest,
    info: web::Path<String>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    if req.cookie("auth").map(|x| x.value().to_owned()) != Some(app_data.correct_secret.clone()) {
        return HttpResponse::Forbidden().finish();
    }

    let id = info.into_inner();

    let r = db::undelete_mailbox(&app_data.db_file, &id);

    match r {
        Ok(opt_mbx) => match opt_mbx {
            Some(mbx) => HttpResponse::Ok().json(MailboxResponse { mailbox: mbx }),
            None => HttpResponse::NotFound().body(""),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/admin/api/mailboxes")]
async fn create_mailbox_handler(
    req: HttpRequest,
    payload: web::Json<AddMailboxRequest>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    if req.cookie("auth").map(|x| x.value().to_owned()) != Some(app_data.correct_secret.clone()) {
        return HttpResponse::Forbidden().finish();
    }

    let req: AddMailboxRequest = payload.into_inner();

    let r = db::create_mailbox_from_request(&app_data.db_file, req.mailbox);

    match r {
        Ok(mbx) => HttpResponse::Ok().json(MailboxResponse { mailbox: mbx }),
        Err(e) => match e {
            SqliteFailure(xx, _) => match xx.code {
                ConstraintViolation => HttpResponse::Conflict().body(""),
                _ => HttpResponse::InternalServerError().body(e.to_string()),
            },
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[get("/admin/api/random-local-parts")]
async fn random_local_parts_handler(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> impl Responder {
    if req.cookie("auth").map(|x| x.value().to_owned()) != Some(app_data.correct_secret.clone()) {
        return HttpResponse::Forbidden().finish();
    }

    HttpResponse::Ok().json(random_local_parts::generate())
}

fn required_env_var(name: &str) -> String {
    match optional_env_var(name) {
        None => {
            panic!("Required environment variable {} is missing", name)
        }
        Some(s) => s,
    }
}

fn optional_env_var(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(s) => Some(s),
        Err(NotPresent) => None,
        Err(NotUnicode(_)) => panic!(
            "Environment variable {} is present, but not valid UTF-8",
            name
        ),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let correct_secret = required_env_var("AUTH_SECRET");
    let db_file = required_env_var("SQLITE_DB_FILE");
    let bind_address = optional_env_var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port: u16 = required_env_var("PORT").parse::<u16>().unwrap();

    let db_data = web::Data::new(AppData {
        correct_secret,
        db_file,
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db_data.clone())
            .service(slash_handler)
            .service(ping_handler)
            .service(preflight_handler)
            .service(list_mailboxes_handler)
            .service(get_mailbox_handler)
            .service(create_mailbox_handler)
            .service(delete_mailbox_handler)
            .service(undelete_mailbox_handler)
            .service(random_local_parts_handler)
            .service(auth_login_handler)
            .service(auth_logout_handler)
    })
    .bind((bind_address, port))?
    .run()
    .await
}
