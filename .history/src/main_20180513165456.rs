#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)] 

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod user;

#[get("/")]
fn index() -> String{
    let a = user::User::new(String::from("test"), String::from("world"));
    a.name
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
