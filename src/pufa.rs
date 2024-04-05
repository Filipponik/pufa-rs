use regex::Regex;
use chrono;
use base64::prelude::*;

pub enum PufaError {
    UuidRequestFailed,
    UuidParseFailed,
    WordRequestFailed,
    WordParseFailed
}

impl PufaError {
    pub fn to_string(&self) -> String {
        match &self {
            PufaError::UuidRequestFailed => "uuid_request_failed",
            PufaError::UuidParseFailed => "uuid_parse_failed",
            PufaError::WordRequestFailed => "word_request_failed",
            PufaError::WordParseFailed => "word_parse_failed",
        }.to_string()
    }
}

pub async fn get_result() -> Result<String, PufaError> {
    let result: String = get_current_uuid().await?;
    let uuid: String = get_uuid_from_js(&result)?;
    let date: String = get_date_formatted();
    get_current_word(&uuid, &date).await
}

fn get_date_formatted() -> String {
    chrono::offset::Utc::now().format("%Y-%m-%d").to_string()
}

async fn get_current_uuid() -> Result<String, PufaError> {
    reqwest::get("https://pufa.afup.org/js/instanceConfiguration.js")
        .await
        .map_err(|_| PufaError::UuidRequestFailed)?
        .text()
        .await
        .map_err(|_| PufaError::UuidParseFailed)
}

async fn get_current_word(uuid: &str, date: &str) -> Result<String, PufaError>
{
    let base64_encoded_token: String = BASE64_STANDARD.encode(format!("{uuid}-{date}"));
    let url: String = format!("https://pufa.afup.org/mots/{base64_encoded_token}.txt");

    reqwest::get(url)
        .await
        .map_err(|_| PufaError::WordRequestFailed)?
        .text()
        .await
        .map_err(|_| PufaError::WordParseFailed)
}

fn get_uuid_from_js(js_code: &str) -> Result<String, PufaError>
{
    let matched = Regex::new(r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}")
        .unwrap()
        .find(js_code);

    match matched {
        Some(value) => Ok(value.as_str().to_string()),
        _ => Err(PufaError::UuidRequestFailed),
    }
}