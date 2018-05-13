use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use rocket::request::FromRequest;
use diesel::MysqlConnection;
use std::ops::Deref;
use rocket::{Outcome, Request, State};

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
