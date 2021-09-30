use rocket::request::FromRequest;
use rocket::{Request, request};
use rocket::outcome::Outcome;
use rocket::http::Status;

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a> for Token {
    type Error = ApiTokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        match token {
            Some(token) => {
                // check validity
                Outcome::Success(Token(token.to_string()))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}