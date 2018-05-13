#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde_derive;

mod models;
mod schema;

use rocket_contrib::Value;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use rocket_contrib::Json;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use std::ops::Deref;

use self::diesel::prelude::*;
use self::models::*;

#[get("/")]
fn index(conn: DbConn) -> String {
    use schema::users::dsl::*;

    let all_users = users.load::<User>(&*conn).expect("BOOM");
    format!("{}", all_users.len())
}

#[get("/users/add/<usernamex>")]
fn add_user(usernamex: String, conn: DbConn) -> Json<Value> {
    use schema::users::dsl::*;

    let u = &usernamex[..];
    Json(json!(create_user(&*conn, u, "asd")))
}

pub fn create_user<'a>(conn: &MysqlConnection, username: &'a str, token: &'a str) -> User {
    use schema::users;

    let new_user = NewUser { username, token };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post")
}

fn main() {
    // dotenv::dotenv().ok();
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, add_user])
        .launch();
}

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn init_pool() -> MysqlPool {
    let database_url = dotenv!("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
