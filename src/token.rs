use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::anyhow;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::{
    blocking::Client as BlockingClient,
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client, StatusCode,
};
use rocket::serde::json::serde_json::json;
use serde::Deserialize;
use serde_json::Value;

static SESSION_URL: &str = "https://public-ubiservices.ubi.com/v3/profiles/sessions";
static BASIC_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
static BASIC_REFRESH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/refresh";

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct TokenResponse<'t> {
    access_token: &'t str,
    refresh_token: &'t str,
}

#[derive(Deserialize, Debug)]
struct TokenPayload {
    rat: u64,
    exp: u64,
}

#[derive(Default, Debug)]
pub struct Token {
    pub access_token: String,
    refresh_token: String,
    pub refresh_at: u64,
    expires_at: u64,
}

impl<'t> From<(TokenResponse<'t>, TokenPayload)> for Token {
    fn from((res, payload): (TokenResponse, TokenPayload)) -> Self {
        Token {
            access_token: res.access_token.into(),
            refresh_token: res.refresh_token.into(),
            refresh_at: payload.rat,
            expires_at: payload.exp,
        }
    }
}

impl Token {
    pub async fn auth() -> anyhow::Result<Token> {
        let client = Client::new();

        let ticket = Self::session_ticket_request(&client).await?.unwrap();

        Self::token_request(&client, ticket).await
    }

    async fn session_ticket_request(client: &Client) -> anyhow::Result<Option<String>> {
        let auth_string = include_str!("../auth.key");
        let auth_string = auth_string.replace("\n", "");

        let mut creds = auth_string.split_terminator(':');
        let user = creds.next().unwrap();
        let pass = creds.next().unwrap();

        let res = client
            .post(SESSION_URL)
            .header(CONTENT_TYPE, "application/json")
            .header("Ubi-AppId", "86263886-327a-4328-ac69-527f0d20a237")
            .header(
                "User-Agent",
                "MapRank Plugin / hellkvistoskar@protonmail.com",
            )
            .basic_auth(user, Some(pass))
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        if status != StatusCode::OK {
            return Err(anyhow!("ERROR: failed to get session ticket: {}", text));
        }

        let json: Value = serde_json::from_str(&text)?;

        match json.get("ticket") {
            Some(ticket) => Ok(Some(ticket.as_str().unwrap().into())),
            None => Ok(None),
        }
    }

    async fn token_request(client: &Client, ticket: String) -> anyhow::Result<Token> {
        let auth = format!("ubi_v1 t={}", ticket);

        let body = json!({
            "audience": "NadeoLiveServices",
        });

        let res = client
            .post(BASIC_AUTH_URL)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        if status != StatusCode::OK {
            return Err(anyhow!("ERROR: Failed to fetch access token: {}", text));
        }

        let token_response: TokenResponse = serde_json::from_str(&text)?;
        let token_payload = Self::decode_payload(&token_response)?;

        let token = Token::from((token_response, token_payload));

        Ok(token)
    }

    fn decode_payload(token_response: &TokenResponse) -> anyhow::Result<TokenPayload> {
        let mut split = token_response.access_token.split_terminator('.');
        let _ = split.next();

        if let Some(encoded_payload) = split.next() {
            let bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
                .decode(encoded_payload)?;
            let payload: TokenPayload = serde_json::from_slice(&bytes)?;
            return Ok(payload);
        }

        Err(anyhow!("Failed to decode token payload"))
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let client = BlockingClient::new();
        let authorization = format!("nadeo_v1 t={}", self.refresh_token);

        let res = client
            .post(BASIC_REFRESH_URL)
            .header(AUTHORIZATION, authorization)
            .send()?;

        let status = res.status();
        let text = res.text()?;

        if status != StatusCode::OK {
            return Err(anyhow!("Failed to refresh access token"));
        }

        let token_response: TokenResponse = serde_json::from_str(&text)?;
        let token_payload = Self::decode_payload(&token_response)?;

        self.access_token = token_response.access_token.into();
        self.refresh_token = token_response.refresh_token.into();
        self.refresh_at = token_payload.rat;
        self.expires_at = token_payload.exp;

        Ok(())
    }

    pub fn as_header(&self) -> String {
        format!("nadeo_v1 t={}", self.access_token)
    }
}

pub fn run_token_thread(token: &Arc<Mutex<Token>>) {
    let token_clone = Arc::clone(&token);
    thread::spawn(move || loop {
        let mut refresh_token_at = None;

        if let Ok(t) = token_clone.lock() {
            refresh_token_at = Some(t.refresh_at);
        }

        if let Some(rat) = refresh_token_at {
            let now = Utc::now();

            let time_to_refresh = NaiveDateTime::from_timestamp_millis(rat as i64 * 1000).unwrap();
            let time_to_refresh = DateTime::from_utc(time_to_refresh, Utc);

            // NOTE: Getting the delta time should not fail unless Nadeo's authentication services
            // are offline, so we have to keep this failsafe here just in case.
            let timeout = match (time_to_refresh - now).to_std() {
                Ok(delta_time) => delta_time,
                Err(e) => {
                    eprintln!("ERROR: failed to get token lifetime: {e:?}");
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
}
