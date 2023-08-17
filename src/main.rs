#[macro_use]
extern crate rocket;
mod records;
mod routes;
mod token;

use lazy_static::lazy_static;
use reqwest::Client;
use token::Token;

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 1337,
        ..Default::default()
    };

    rocket::custom(&config)
        .register("/", catchers![routes::not_found, routes::default])
        .manage(Token::auth().unwrap())
        .mount("/", routes![routes::np])
}
