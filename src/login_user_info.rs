use rocket::request::FromRequest;
use rocket::{Request, request};
use rocket::outcome::Outcome;
use rocket::http::Status;

pub struct LoginUserInfo {
    token: String,
    userId: i64,
    appId: i32
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginUserInfo {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("accessToken");
        match token {
            Some(token) => {
                let loginUserInfo = LoginUserInfo {
                    token: token.to_string(),
                    userId: 0,
                    appId: 0
                };
                // check validity
                Outcome::Success(loginUserInfo)
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}