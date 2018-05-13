#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

mod schema;
mod models;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::mysql::MysqlConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

use self::models::*;
use self::diesel::prelude::*;


#[get("/")]
fn index(conn: DbConn) -> String {
    use schema::users::dsl::*;

    let all_users = users.load::<User>(&*conn).expect("BOOM");
    format!("{}", all_users.len())
}

#[get("/users/add/<username>")]
fn add_user(username: String, conn: DbConn) -> String {
    use schema::users::dsl::*;

    users.
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
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
