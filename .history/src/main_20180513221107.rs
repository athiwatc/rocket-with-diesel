#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate diesel;

mod user;

#[get("/")]
fn index() -> String {
    let a = user::User::new(String::from("test"), String::from("world"));
    a.name
}

fn main() {
    dotenv::dotenv().ok();
    
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index])
        .launch();
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

static DATABASE_URL: &'static str = env!("DATABASE_URL");

fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
