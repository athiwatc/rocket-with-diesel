#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
use rocket_contrib::Json;
use rocket_contrib::Value;

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
use self::diesel::prelude::*;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod database;
use database::connection::init_pool;
use database::connection::DbConn;
use database::create::create_user;
use database::models::*;

use uuid::Uuid;

#[get("/")]
fn index(conn: DbConn) -> Json<Value> {
    use database::schema::users::dsl::*;

    let all_users = users.load::<User>(&*conn).expect("BOOM");
    Json(json!(all_users))
}

#[get("/users/add/<username>")]
fn add_user(username: String, conn: DbConn) -> Json<Value> {
    create_user(&conn, &username[..], &Uuid::new_v4().to_string()[..]);
    Json(json!({ "username": &username }))
}

#[get("/bench/<count_opt>")]
fn bench(count_opt: Option<i32>, conn: DbConn) -> String {
    
    let count = match count_opt {
        Some(c) => c,
        None => 100
    };

    for _ in 1..count {
        create_user(&conn, "bench", &Uuid::new_v4().to_string()[..]);
    }

    format!("Bench: {} times.", count)
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, add_user, bench])
        .launch();
}
