use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::{Outcome, Request, State};
use std::ops::Deref;
use diesel::connection::Connection;

type DatabasePool<T> = Pool<ConnectionManager<T>>;

pub fn init_pool<T>() -> DatabasePool<T> where T: Connection, T: 'static {
    let database_url = dotenv!("DATABASE_URL");
    let manager = ConnectionManager::<T>::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct DbConn<T>(pub PooledConnection<ConnectionManager<T>>) where T: Connection, T: 'static;

impl<'a, 'r, T> FromRequest<'a, 'r> for DbConn<T> where T: Connection  {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<DatabasePool<T>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl<T> Deref for DbConn<T> where T: Connection {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
