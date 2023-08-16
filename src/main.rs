#[macro_use]
extern crate rocket;
mod records;
mod routes;
mod token;

use lazy_static::lazy_static;
use reqwest::Client;
use token::Token;

lazy_static! {
    static ref CONFIG: rocket::Config = rocket::Config {
        port: 1337,
        ..Default::default()
    };
    pub static ref TOKEN: Token = Token::auth().unwrap();
    pub static ref CLIENT: Client = Client::new();
    pub static ref CREDS: (String, String) = {
        let mut creds = include_str!("../auth.key").split_terminator(':');
        (creds.next().unwrap().into(), creds.next().unwrap().into())
    };
}

#[launch]
fn rocket() -> _ {
    rocket::custom(&*CONFIG)
        .register("/", catchers![routes::not_found, routes::default])
        .mount("/", routes![routes::np])
}
