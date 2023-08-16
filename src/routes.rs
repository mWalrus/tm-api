use rocket::{
    http::Status,
    response::status,
    serde::json::{serde_json::json, Value},
    Request,
};

use crate::records;

#[catch(default)]
pub fn default(status: Status, _req: &Request<'_>) -> status::Custom<Value> {
    status::Custom(status, json!({"message": "Something went wrong"}))
}

#[catch(404)]
pub fn not_found() -> status::Custom<&'static str> {
    status::Custom(Status::NotFound, "Not found")
}

#[get("/np/map/<muid>")]
pub async fn np(muid: &str) -> status::Custom<Value> {
    let player_count = records::get_player_count(muid).await.unwrap();
    status::Custom(Status::Ok, json!({ "player_count": player_count }))
}
