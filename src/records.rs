use std::sync::{Arc, Mutex};

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
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> anyhow::Result<Option<u32>> {
    let access_token = if let Ok(token) = token.lock() {
        Some(token.as_header())
    } else {
        None
    };

    if let Some(access_token) = access_token {
        let url = format!("https://live-services.trackmania.nadeo.live/api/token/leaderboard/group/Personal_Best/map/{muid}/surround/1/1?onlyWorld=true&score=4294967295");

        let req = client
            .get(url)
            .header(AUTHORIZATION, access_token)
            .build()?;

        let res = client.execute(req).await?;
        let text = res.text().await?;
        let records: RecordResponse = serde_json::from_str(&text)?;

        let mut count = records.tops[0].top[0].position;
        // this means that there are really no players who have set records on this map
        if records.tops[0].top.len() == 1 && count == 1 {
            count = 0;
        }

        return Ok(Some(count));
    }
    Ok(None)
}
