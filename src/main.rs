#[macro_use]
extern crate rocket;
use base64::DecodeError;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::Redirect,
    serde::json::{json, Value},
    Request,
};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum BasicAuthError {
    MissingHeader,
    InvalidHeaderFormat,
    InvalidBase64Encoding(DecodeError),
    InvalidUtf8Encoding(FromUtf8Error),
}

pub struct BasicAuth {
    pub username: String,
    pub password: String,
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
            Ok(auth) => Outcome::Success(auth),
            Err(BasicAuthError::MissingHeader) | Err(BasicAuthError::InvalidHeaderFormat) => {
                Outcome::Failure((Status::Unauthorized, BasicAuthError::MissingHeader))
            }
            Err(error) => Outcome::Failure((Status::BadRequest, error)),
        }
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "message": "Not Found",
    })
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({
        "message": "Unauthorized",
    })
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/api")
}

#[get("/api")]
fn api() -> Value {
    json!({
        "message": "Welcome to the API",
    })
}

#[get("/api/persons")]
fn get_persons(_auth: BasicAuth) -> Value {
    json!([{
        "name": "raul",
    }, {
        "name":"carla",
    }])
}

#[get("/api/persons/<id>")]
fn get_person(id: i32) -> Value {
    json!({
        "id": id,
        "name": "raul",
    })
}

#[post("/api/persons", format = "json")]
fn new_person() -> Value {
    json!({
        "message": "new person created",
    })
}

#[put("/api/persons/<id>", format = "json")]
fn update_person(id: i32) -> Value {
    json!({
        "mesage": "person updated",
        "name": "raul",
        "id": id,
    })
}

#[delete("/api/persons/<id>")]
fn delete_person(id: i32) -> Value {
    json!({
        "message": "person deleted",
        "id": id,
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                index,
                api,
                get_persons,
                get_person,
                new_person,
                update_person,
                delete_person,
            ],
        )
        .register("/", catchers![not_found, unauthorized])
        .launch()
        .await;
}
