use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

use crate::{CLIENT, TOKEN};

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

pub async fn get_player_count(muid: &str) -> anyhow::Result<u32> {
    let url = format!("https://live-services.trackmania.nadeo.live/api/token/leaderboard/group/Personal_Best/map/{muid}/surround/1/1?onlyWorld=true&score=4294967295");

    let req = CLIENT
        .get(url)
        .header(AUTHORIZATION, TOKEN.as_header())
        .build()?;

    let text = CLIENT.execute(req).await?.text().await?;
    let res: RecordResponse = serde_json::from_str(&text)?;

    let count = res.tops[0].top[0].position;
    Ok(count)
}
