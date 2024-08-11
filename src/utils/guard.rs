use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthGuard {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(Status::Unauthorized);
        }

        let token = keys[0].replace("Bearer ", "");
        let decoding_key = DecodingKey::from_secret("secret".as_ref());

        match decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::HS256)) {
            Ok(token_data) => Outcome::Success(AuthGuard {
                user_id: token_data.claims.sub,
            }),
            Err(_) => Outcome::Forward(Status::Unauthorized),
        }
    }
}