use std::sync::{Arc, Mutex};

use reqwest::Client;
use rocket::{
    http::Status,
    response::status,
    serde::json::{serde_json::json, Value},
    Request, State,
};

use crate::{records, token::Token};

#[catch(default)]
pub fn default(status: Status, _req: &Request<'_>) -> status::Custom<Value> {
    status::Custom(
        status,
        json!({ "message": format!("ERROR {status}: Something went wrong") }),
    )
}

#[catch(404)]
pub fn not_found() -> status::Custom<&'static str> {
    status::Custom(Status::NotFound, "Not found")
}

#[get("/np/map/<muid>")]
pub async fn np(
    muid: &str,
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> status::Custom<Value> {
    let player_count = records::get_player_count(muid, token, client)
        .await
        .unwrap_or_default();
    status::Custom(Status::Ok, json!({ "player_count": player_count }))
}

#[get("/pos/<muid>/<score>")]
pub async fn pos(
    muid: &str,
    score: u64,
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> status::Custom<Value> {
    let pos = records::get_map_pos_at(muid, score, token, client)
        .await
        .unwrap_or_default();
    status::Custom(Status::Ok, json!({ "position": pos }))
}
