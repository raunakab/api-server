use crate::schema::*;
use serde::{
    Serialize,
    Deserialize,
};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub_id: i32,
    first_name: String,
    last_name: String,
    email: String,
    created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
