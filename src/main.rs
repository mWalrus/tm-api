#[macro_use]
extern crate rocket;
mod records;
mod routes;
mod token;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Client;
use token::Token;

#[launch]
async fn rocket() -> _ {
    let config = rocket::Config {
        port: 8000,
        ..Default::default()
    };

    let token = Arc::new(Mutex::new(Token::auth().await.unwrap()));
    let token_clone = Arc::clone(&token);
    thread::spawn(move || loop {
        let mut rat = None;

        if let Ok(t) = token_clone.lock() {
            rat = Some(t.rat);
        }

        if let Some(rat) = rat {
            let now = Utc::now();

            let later = NaiveDateTime::from_timestamp_millis(rat as i64 * 1000).unwrap();
            let later = DateTime::from_utc(later, Utc);

            let diff = later - now;
            let diff = diff.to_std().unwrap();

            thread::sleep(diff);

            if let Ok(t) = &mut token_clone.lock() {
                match t.refresh() {
                    Ok(_) => println!("INFO: Refreshed access token!"),
                    Err(e) => eprintln!("ERROR: Failed to refresh access token: {e:?}"),
                }
            }
        }
    });

    rocket::custom(&config)
        .register("/", catchers![routes::not_found, routes::default])
        .manage(token)
        .manage(Client::new())
        .mount("/", routes![routes::np])
}
