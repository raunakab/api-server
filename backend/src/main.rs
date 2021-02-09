#![allow(unused)]

use actix_web::{
    web,
    App,
    HttpServer,
    dev::{
        ServiceRequest,
        HttpResponseBuilder,
    },
    Error,
    error,
    HttpResponse,
};

pub async fn hello_world() -> Result<HttpResponse, Error> {
    return Ok(
        web::block(move || -> Result<String, ()> {
            return Ok(String::from("Hello, world!\n"));
        })
            .await
            .map::<HttpResponse, fn(String) -> HttpResponse>(|msg: String| -> HttpResponse {
                return HttpResponse::Ok().json(msg);
            })
            .map_err(|_: error::BlockingError<()>| -> HttpResponseBuilder {
                return HttpResponse::InternalServerError();
            })?
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(move || {
        return App::new()
            .route("/", web::get()  .to(hello_world));
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;
}
