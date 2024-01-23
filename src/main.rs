#[macro_use]
extern crate rocket;
mod records;
mod routes;
mod token;

use std::sync::{Arc, Mutex};

use reqwest::Client;
use token::{run_token_thread, Token};

#[launch]
async fn rocket() -> _ {
    let config = rocket::Config {
        port: 8000,
        ..Default::default()
    };

    let token = Arc::new(Mutex::new(Token::auth().await.unwrap()));
    run_token_thread(&token);

    rocket::custom(&config)
        .register("/", catchers![routes::not_found, routes::default])
        .manage(token)
        .manage(Client::new())
        .mount("/", routes![routes::np, routes::pos])
}
