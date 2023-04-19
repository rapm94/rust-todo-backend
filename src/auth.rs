use base64::DecodeError;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::Unauthorized,
    Request,
};
use std::string::FromUtf8Error;

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum BasicAuthError {
    MissingHeader,
    InvalidHeaderFormat,
    InvalidBase64Encoding(DecodeError),
    InvalidUtf8Encoding(FromUtf8Error),
    Unauthorized,
}
impl BasicAuth {
    pub fn from_authorization_header(header: Option<&str>) -> Result<BasicAuth, BasicAuthError> {
        let header = header.ok_or(BasicAuthError::MissingHeader)?;
        let mut parts = header.split_whitespace();

        if parts.next() != Some("Basic") {
            return Err(BasicAuthError::InvalidHeaderFormat);
        }

        let base64_str = parts.next().ok_or(BasicAuthError::InvalidHeaderFormat)?;

        BasicAuth::from_base64_encoded(base64_str)
    }

    fn from_base64_encoded(base64_str: &str) -> Result<BasicAuth, BasicAuthError> {
        let decoded = base64::decode(base64_str).map_err(BasicAuthError::InvalidBase64Encoding)?;
        let decoded_str =
            String::from_utf8(decoded).map_err(BasicAuthError::InvalidUtf8Encoding)?;
        let mut parts = decoded_str.splitn(2, ':');
        let username = parts
            .next()
            .ok_or(BasicAuthError::InvalidHeaderFormat)?
            .trim()
            .to_string();
        let password = parts
            .next()
            .ok_or(BasicAuthError::InvalidHeaderFormat)?
            .trim()
            .to_string();
        Ok(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = BasicAuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match BasicAuth::from_authorization_header(auth_header) {
            Ok(auth) => {
                if auth.username == "foo" && auth.password == "bar" {
                    Outcome::Success(auth)
                } else {
                    Outcome::Failure((Status::Unauthorized, BasicAuthError::Unauthorized))
                }
            }
            Err(BasicAuthError::MissingHeader) | Err(BasicAuthError::InvalidHeaderFormat) => {
                Outcome::Failure((Status::Unauthorized, BasicAuthError::MissingHeader))
            }
            Err(error) => Outcome::Failure((Status::BadRequest, error)),
        }
    }
}
