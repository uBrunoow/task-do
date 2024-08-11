use rocket::Route;
use rocket::serde::json::Json;
use crate::DbConn;
use crate::user::service;
use crate::user::models::user::{GetUser, RegisterUser, RegisterUserResponse, LoginUserResponse, LoginUser};
use crate::utils::guard::AuthGuard;

#[get("/users")]
pub async fn get_users(_auth: AuthGuard, conn: DbConn) -> Json<Vec<GetUser>> {
    service::get_users(conn).await
}

#[get("/users/<user_id>")]
pub async fn get_user_by_id(_auth: AuthGuard, conn: DbConn, user_id: i32) -> Json<GetUser> {
    service::get_user_by_id(conn, user_id).await
}

#[post("/register", format = "json", data = "<user>")]
pub async fn register_user(conn: DbConn, user: Json<RegisterUser>) -> Json<RegisterUserResponse> {
    service::register_user(conn, user.into_inner()).await
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login_user(conn: DbConn, user: Json<LoginUser>) -> Json<LoginUserResponse> {
    service::login_user(conn, user.into_inner()).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_users, get_user_by_id, register_user, login_user]
}