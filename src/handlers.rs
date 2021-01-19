use actix_web::Responder;
use crate::models::{
    User,
    NewUser,
};
use crate::schema::users::dsl::*;
use crate::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{
    web,
    Error,
    HttpResponse,
    dev,
    error,
};
use diesel::dsl::{
    delete,
    insert_into,
};
use serde::{
    Deserialize,
    Serialize
};
use std::vec::Vec;
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    first_name: String,
    last_name: String,
    email: String,
}

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    return Ok(
        web::block(move || -> Result<Vec<User>, diesel::result::Error> {
            let connection = db.get().unwrap();
            let db_users: Vec<User> = users.load::<User>(&connection)?;

            return Ok(db_users);
        })
        .await
        .map::<HttpResponse, fn(Vec<User>) -> HttpResponse>(|db_users: Vec<User>| -> HttpResponse {
            return HttpResponse::Ok().json(db_users);
        })
        .map_err::<dev::HttpResponseBuilder, fn(error::BlockingError<diesel::result::Error>) -> dev::HttpResponseBuilder>
        (|_: error::BlockingError<diesel::result::Error>| -> dev::HttpResponseBuilder {
            return HttpResponse::InternalServerError();
        })?
    );
}

pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    return Ok(
        web::block(move || -> Result<User, diesel::result::Error> {
            let connection = db.get().unwrap();
            let user: User = users.find(user_id.into_inner()).get_result::<User>(&connection)?;

            return Ok(user);
        })
        .await
        .map::<HttpResponse, fn(User) -> HttpResponse>(|user_id: User| -> HttpResponse {
            return HttpResponse::Ok().json(user_id);
        })
        .map_err::<dev::HttpResponseBuilder, fn(error::BlockingError<diesel::result::Error>) -> dev::HttpResponseBuilder>
        (|_: error::BlockingError<diesel::result::Error>| -> dev::HttpResponseBuilder {
            return HttpResponse::InternalServerError();
        })?
    );
}

pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    return Ok(
        web::block(move || -> Result<User, diesel::result::Error> {
            let connection = db.get().unwrap();
            let new_user = NewUser {
                first_name: &item.first_name,
                last_name: &item.last_name,
                email: &item.email,
                created_at: chrono::Local::now().naive_local(),
            };
            let res = insert_into(users).values(&new_user).get_result(&connection)?;

            return Ok(res);
        })
        .await
        .map::<HttpResponse, fn(User) -> HttpResponse>(|user: User| -> HttpResponse {
            return HttpResponse::Created().json(user);
        })
        .map_err::<dev::HttpResponseBuilder, fn(error::BlockingError<diesel::result::Error>) -> dev::HttpResponseBuilder>
        (|_: error::BlockingError<diesel::result::Error>| -> dev::HttpResponseBuilder {
            return HttpResponse::InternalServerError();
        })?
    );
}

pub async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    return Ok(
        web::block(move || -> Result<usize, diesel::result::Error> {
            let connection = db.get().unwrap();
            let count: usize = delete(users.find(user_id.into_inner())).execute(&connection)?;

            return Ok(count);
        })
        .await
        .map::<HttpResponse, fn(usize) -> HttpResponse>(|user: usize| -> HttpResponse {
            return HttpResponse::Ok().json(user);
        })
        .map_err::<dev::HttpResponseBuilder, fn(error::BlockingError<diesel::result::Error>) -> dev::HttpResponseBuilder>(|_: error::BlockingError<diesel::result::Error>| -> dev::HttpResponseBuilder {
            return HttpResponse::InternalServerError();
        })?,
    );
}
