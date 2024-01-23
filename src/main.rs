#[macro_use]
extern crate rocket;
mod records;
mod routes;
mod token;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
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

            let time_to_refresh = NaiveDateTime::from_timestamp_millis(rat as i64 * 1000).unwrap();
            let time_to_refresh = DateTime::from_utc(time_to_refresh, Utc);

            // NOTE: Getting the delta time should not fail unless Nadeo's authentication services
            // are offline, so we have to keep this failsafe here just in case.
            let timeout = match (time_to_refresh - now).to_std() {
                Ok(delta_time) => delta_time,
                Err(e) => {
                    eprintln!("ERROR: failed to get token age: {e:?}");
                    Duration::from_secs(60)
                }
            };

            thread::sleep(timeout);

            if let Ok(t) = &mut token_clone.lock() {
                match t.refresh() {
                    Ok(_) => println!("INFO: Refreshed access token!"),
                    Err(e) => eprintln!("ERROR: {e:?}"),
                }
            }
        }
    });

    rocket::custom(&config)
        .register("/", catchers![routes::not_found, routes::default])
        .manage(token)
        .manage(Client::new())
        .mount("/", routes![routes::np, routes::pos])
}
