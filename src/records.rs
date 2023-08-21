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
    position: u64,
}

pub async fn get_player_count(
    muid: &str,
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> anyhow::Result<Option<u64>> {
    let records = make_record_request(muid, 4294967295, 1, token, client).await?;

    if let Some(records) = records {
        let mut count = records.tops[0].top[0].position;
        // this means that there are really no players who have set records on this map
        if records.tops[0].top.len() == 1 && count == 1 {
            count = 0;
        }
        Ok(Some(count))
    } else {
        Ok(None)
    }
}

pub async fn get_map_pos_at(
    muid: &str,
    score: u64,
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> anyhow::Result<Option<u64>> {
    let records = make_record_request(muid, score, 0, token, client).await?;
    if let Some(records) = records {
        let count = records.tops[0].top[0].position;
        Ok(Some(count))
    } else {
        Ok(None)
    }
}

async fn make_record_request(
    muid: &str,
    score: u64,
    surround: u8,
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> anyhow::Result<Option<RecordResponse>> {
    let access_token = if let Ok(token) = token.lock() {
        Some(token.as_header())
    } else {
        None
    };

    if let Some(access_token) = access_token {
        let url = format!("https://live-services.trackmania.nadeo.live/api/token/leaderboard/group/Personal_Best/map/{muid}/surround/{surround}/{surround}?onlyWorld=true&score={score}");

        let req = client
            .get(url)
            .header(AUTHORIZATION, access_token)
            .build()?;

        let res = client.execute(req).await?;
        let text = res.text().await?;
        let records: RecordResponse = serde_json::from_str(&text)?;

        return Ok(Some(records));
    }
    Ok(None)
}
