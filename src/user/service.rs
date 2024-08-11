use rocket::serde::json::Json;
use crate::user::models::user::{GetUser, RegisterUser, RegisterUserResponse, LoginUserResponse, LoginUser};
use crate::user::repository;
use crate::DbConn;

pub async fn get_users(conn: DbConn) -> Json<Vec<GetUser>> {
    conn.run(|c| {
        repository::get_all_users(c).map(Json).expect("Error loading users")
    }).await
}

pub async fn get_user_by_id(conn: DbConn, user_id: i32) -> Json<GetUser> {
    conn.run(move |c| {
        repository::get_user_by_id(c, user_id).map(Json).expect("Error loading user")
    }).await
}


pub async fn register_user(conn: DbConn, user: RegisterUser) -> Json<RegisterUserResponse> {
    conn.run(move |c| {
        repository::register_user(c, &user.username, &user.email, &user.password)
            .map(Json)
            .expect("Error registering user")
    }).await
}

pub async fn login_user(conn: DbConn, user: LoginUser) -> Json<LoginUserResponse> {
    conn.run(move |c| {
        repository::login_user(c, user)
            .map(Json)
            .expect("Error logging in user")
    }).await
}

// pub async fn update_user(conn: DbConn, user: UpdateUser, user_id: i32) -> Json<GetUser> {
//     conn.run(move |c| {
//         repository::update_user(c, &user.username, &user.email, &user.password, user_id)
//             .map(Json)
//             .expect("Error updating user")
//     }).await
// }