use reqwest::{header::AUTHORIZATION, Client};
use rocket::State;
use serde::Deserialize;

use crate::token::Token;

#[derive(Deserialize, Debug)]
pub struct RecordResponse {
    tops: Vec<Tops>,
}

#[derive(Deserialize, Debug)]
pub struct Tops {
    top: Vec<Top>,
}

#[derive(Deserialize, Debug)]
pub struct Top {
    position: u32,
}

pub async fn get_player_count(
    muid: &str,
    token: &State<Token>,
    client: &State<Client>,
) -> anyhow::Result<u32> {
    let url = format!("https://live-services.trackmania.nadeo.live/api/token/leaderboard/group/Personal_Best/map/{muid}/surround/1/1?onlyWorld=true&score=4294967295");

    let req = client
        .get(url)
        .header(AUTHORIZATION, token.as_header())
        .build()?;

    let text = client.execute(req).await?.text().await?;
    let res: RecordResponse = serde_json::from_str(&text)?;

    let count = res.tops[0].top[0].position;
    Ok(count)
}
