use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use crate::user::models::user::{User, GetUser, NewUser, RegisterUserResponse, LoginUser, Claims, LoginUserResponse};
use crate::schema::users::dsl::*;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, EncodingKey};

pub fn get_all_users(conn: &mut PgConnection) -> QueryResult<Vec<GetUser>> {
    users.select((id, username, email)).load::<GetUser>(conn)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> QueryResult<GetUser> {
    users.select((id, username, email)).find(user_id).first::<GetUser>(conn)
}

pub fn register_user(conn: &mut PgConnection, user_name: &str, email_str: &str, password_str: &str) -> QueryResult<RegisterUserResponse> {
    let password_to_hash = password_str.to_string();
    let hashed_password = hash(password_to_hash, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: user_name.to_string(),
        email: email_str.to_string(),
        password: hashed_password.clone(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let user_id: i32 = users
        .select(id)
        .order(id.desc())
        .first(conn)?;

    let response = RegisterUserResponse {
        id: user_id, // Inclua o id aqui
        username: user_name.to_string(),
        email: email_str.to_string(),
    };

    Ok(response)
}


pub fn login_user(conn: &mut PgConnection, login_user: LoginUser) -> Result<LoginUserResponse, String> {
    use crate::schema::users::dsl::{users, email};

    let user_result = users
        .filter(email.eq(&login_user.email))
        .first::<User>(conn);

    match user_result {
        Ok(user) => {
            if verify(&login_user.password, &user.password).unwrap() {
                let expiration = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() + 60 * 60; // 1 hour expiration

                let claims = Claims {
                    sub: user.username.clone(),
                    exp: expiration as usize,
                };

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();

                let response = LoginUserResponse {
                    id: user.id,
                    token,
                    username: user.username,
                    email: user.email,
                    password: user.password,
                };

                Ok(response)
            } else {
                Err("Invalid password".to_string())
            }
        }
        Err(_) => Err("User not found".to_string()),
    }
} 

// pub fn update_user(conn: &mut PgConnection, user_id: i32, user_name: &str, email_str: &str, password_str: &str) -> QueryResult<GetUser> {
//     let password_to_hash = password_str.to_string();
//     let hashed_password = hash(password_to_hash, DEFAULT_COST).unwrap();
//     let new_user = UpdateUser {
//         username: user_name.to_string(),
//         email: email_str.to_string(),
//         password: hashed_password.clone(),
//     };

//     diesel::update(users.find(user_id)).set(&new_user).execute(conn)?;

//     let response = GetUser {
//         id: user_id, // Inclua o id aqui
//         username: user_name.to_string(),
//         email: email_str.to_string(),
//     };

//     Ok(response)
// }