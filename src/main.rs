#[macro_use]
extern crate rocket;
mod auth;
use auth::BasicAuth;
use rocket::{
    response::Redirect,
    serde::json::{json, Value},
};

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
fn get_person(id: i32, _auth: BasicAuth) -> Value {
    json!({
        "id": id,
        "name": "raul",
    })
}

#[post("/api/persons", format = "json")]
fn new_person(_auth: BasicAuth) -> Value {
    json!({
        "message": "new person created",
    })
}

#[put("/api/persons/<id>", format = "json")]
fn update_person(id: i32, _auth: BasicAuth) -> Value {
    json!({
        "mesage": "person updated",
        "name": "raul",
        "id": id,
    })
}

#[delete("/api/persons/<id>")]
fn delete_person(id: i32, _auth: BasicAuth) -> Value {
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
