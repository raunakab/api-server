#![allow(unused)]

#[macro_use]
extern crate diesel;

use actix_web::{
    web,
    App,
    HttpServer,
    dev::ServiceRequest,
    Error,
};
use diesel::prelude::*;
use diesel::r2d2::{
    self,
    ConnectionManager,
};

mod handlers;
mod errors;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: ConnectionManager::<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

    return HttpServer::new(move || {
        return App::new()
        .data(pool.clone())
        .route("/users",      web::get()    .to(handlers::get_users))
        .route("/users/{id}", web::get()    .to(handlers::get_user_by_id))
        .route("/users",      web::post()   .to(handlers::add_user))
        .route("/users/{id}", web::delete() .to(handlers::delete_user));
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
}
