use super::connection::DbConn;
use super::models::NewUser;
use diesel;
use diesel::RunQueryDsl;
use diesel::prelude::*;

pub fn create_user<'a>(conn: &DbConn<SqliteConnection>, username: &'a str, token: &'a str) {
  use database::schema::users;

  let new_user = NewUser { username, token };

  diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&**conn)
    .expect("Error saving new post");
}
