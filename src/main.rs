#[macro_use]
extern crate rocket;
mod records;
mod routes;
mod token;

use reqwest::Client;
use token::Token;

#[launch]
async fn rocket() -> _ {
    let config = rocket::Config {
        port: 1337,
        ..Default::default()
    };

    rocket::custom(&config)
        .register("/", catchers![routes::not_found, routes::default])
        .manage(Token::auth().await.unwrap())
        .manage(Client::new())
        .mount("/", routes![routes::np])
}
