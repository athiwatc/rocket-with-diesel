use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use rocket::request::FromRequest;
use diesel::MysqlConnection;
use std::ops::Deref;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self};

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> MysqlPool {
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

pub fn create_user<'a>(conn: &MysqlConnection, username: &'a str, token: &'a str) {
    use database::schema::users;

    let new_user = NewUser { username, token };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
