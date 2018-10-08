#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate uuid;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use rocket_contrib::{Json, Value};
use self::uuid::Uuid;

mod products;
use products::{Product, CreateProduct};
use products::cache;

#[post("/", data = "<product>")]
fn create(product: Json<CreateProduct>) -> Json<Value> {
    let product = product.into_inner().new();
    Json(json!({"status": "ok"}))
}

#[get("/")]
fn read() -> Json<Value> {
    cache::display_all();
    Json(json!({"status": "ok"}))
}

#[put("/<id>", data = "<product>")]
fn update(id: String, product: Json<Product>) -> Json<Product> {
    product
}

#[delete("/<id>")]
fn delete(id: String) -> Json<Value> {
    let id = Uuid::parse_str(&id).unwrap();
    cache::remove(id);
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
        .mount("/products", routes![create, update, delete, read])
        .launch();
}