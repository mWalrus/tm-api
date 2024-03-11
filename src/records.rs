use std::sync::{Arc, Mutex};

use reqwest::{header::AUTHORIZATION, Client};
use rocket::State;
use serde::Deserialize;

use crate::token::Token;

enum PositionType {
    PlayerCount,
    PlayerPosition,
}

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

    Ok(find_position(records, PositionType::PlayerCount))
}

pub async fn get_map_pos_at(
    muid: &str,
    score: u64,
    token: &State<Arc<Mutex<Token>>>,
    client: &State<Client>,
) -> anyhow::Result<Option<u64>> {
    let records = make_record_request(muid, score, 0, token, client).await?;
    Ok(find_position(records, PositionType::PlayerPosition))
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

fn find_position<'a>(records: Option<RecordResponse>, position_type: PositionType) -> Option<u64> {
    let records = records?;
    let tops = records.tops.get(0)?;
    let top = tops.top.get(0)?;

    let mut pos = top.position;
    match position_type {
        PositionType::PlayerCount => {
            // Usually, when fetching the current score and the surrounding scores, we would get back 3 scores.
            // However, this is not the case when there are no records on a map and we can detect that by checking
            // whether the amount of records are 1. This means that, in reality, there are no records.
            // So here, we just correct the reported player count to what it should be in this case, which is 0.
            if tops.top.len() == 1 && pos == 1 {
                pos = 0;
            }
        }
        _ => {}
    }

    Some(pos)
}
