#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub token: String,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}