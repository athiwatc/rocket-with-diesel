#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)] 

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> String{
    let a = User::new("test", "world");
    a.name
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
