use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct GetUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"] 
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginUserResponse {
    pub id: i32,
    pub token: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

// #[derive(Deserialize, AsChangeset)]
// pub struct UpdateUser {
//     pub username: String,
//     pub email: String,
//     pub password: String,
// }