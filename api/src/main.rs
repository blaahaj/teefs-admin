mod db;
mod random_local_parts;
mod types;

use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use env_logger::Env;
use rusqlite::{self, Error::SqliteFailure, ErrorCode::ConstraintViolation};
use serde::Serialize;

use std::env;
use std::env::VarError::{NotPresent, NotUnicode};

use crate::types::{AddMailboxRequest, MailboxResponse, MailboxesResponse};

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

#[get("/api/mailboxes")]
async fn list_mailboxes_handler(db_file: web::Data<String>) -> impl Responder {
    match db::list_mailboxes(&db_file) {
        Ok(mailboxes) => HttpResponse::Ok().json(MailboxesResponse { mailboxes }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/mailboxes/{id}")]
async fn get_mailbox_handler(
    info: web::Path<String>,
    db_file: web::Data<String>,
) -> impl Responder {
    let id = info.into_inner();

    match db::get_mailbox(&db_file, &id) {
        Ok(mailbox) => match mailbox {
            Some(m) => HttpResponse::Ok().json(MailboxResponse { mailbox: m }),
            None => HttpResponse::NotFound().body(""),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/api/mailboxes/{id}/delete")]
async fn delete_mailbox_handler(
    info: web::Path<String>,
    db_file: web::Data<String>,
) -> impl Responder {
    let id = info.into_inner();

    let r = db::delete_mailbox(&db_file, &id);

    match r {
        Ok(opt_mbx) => match opt_mbx {
            Some(mbx) => HttpResponse::Ok().json(MailboxResponse { mailbox: mbx }),
            None => HttpResponse::NotFound().body(""),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/mailboxes/{id}/undelete")]
async fn undelete_mailbox_handler(
    info: web::Path<String>,
    db_file: web::Data<String>,
) -> impl Responder {
    let id = info.into_inner();

    let r = db::undelete_mailbox(&db_file, &id);

    match r {
        Ok(opt_mbx) => match opt_mbx {
            Some(mbx) => HttpResponse::Ok().json(MailboxResponse { mailbox: mbx }),
            None => HttpResponse::NotFound().body(""),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/mailboxes")]
async fn create_mailbox_handler(
    payload: web::Json<AddMailboxRequest>,
    db_file: web::Data<String>,
) -> impl Responder {
    let req: AddMailboxRequest = payload.into_inner();

    let r = db::create_mailbox_from_request(&db_file, req.mailbox);

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

#[get("/api/random-local-parts")]
async fn random_local_parts_handler() -> impl Responder {
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
    let database_file = required_env_var("SQLITE_DB_FILE");
    let bind_address = optional_env_var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port: u16 = required_env_var("PORT").parse::<u16>().unwrap();

    let db_data = web::Data::new(database_file);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DefaultHeaders::new().add((
                "Access-Control-Allow-Origin",
                required_env_var("ACCESS_CONTROL_ALLOW_ORIGIN"),
            )))
            .app_data(db_data.clone())
            .service(slash_handler)
            .service(ping_handler)
            .service(list_mailboxes_handler)
            .service(get_mailbox_handler)
            .service(create_mailbox_handler)
            .service(delete_mailbox_handler)
            .service(undelete_mailbox_handler)
            .service(random_local_parts_handler)
    })
    .bind((bind_address, port))?
    .run()
    .await
}
