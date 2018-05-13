#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)] 

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;

mod user;

#[get("/")]
fn index() -> String{
    let a = user::User::new(String::from("test"), String::from("world"));
    a.name
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index]).launch();
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("db pool")
}