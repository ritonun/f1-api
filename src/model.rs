use serde::{Deserialize, Serialize};

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
