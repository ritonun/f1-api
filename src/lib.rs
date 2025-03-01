use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

pub struct OpenF1Client {
    base_url: String,
    client: Client,
}

impl OpenF1Client {
    pub fn new() -> OpenF1Client {
        OpenF1Client {
            base_url: "https://api.openf1.org/v1/".to_string(),
            client: Client::new(),
        }
    }

    async fn get_value(&self, endpoint: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?.json::<Value>().await?;
        Ok(response)
    }

    async fn get_vec_value(&self, endpoint: &str) -> Result<Vec<Value>, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<Value>>()
            .await?;
        Ok(response)
    }

    pub async fn get_sessions(&self) -> Result<Vec<Session>, Box<dyn Error>> {
        let response = self.get_vec_value("sessions").await?;
        let sessions: Vec<Session> = serde_json::from_value(Value::Array(response))?;
        Ok(sessions)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    circuit_key: u32,
    circuit_short_name: String,
    country_code: String,
    country_key: u32,
    country_name: String,
    date_end: String,
    date_start: String,
    gmt_offset: String,
    location: String,
    meeting_key: u32,
    session_key: u32,
    session_name: String,
    session_type: String,
    year: u32,
}
