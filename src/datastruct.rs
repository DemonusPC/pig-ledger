use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRequest {
    pub year: i32,
    pub month: u8,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub date: chrono::DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SqlResult {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Currency {
    pub code: String,
    pub numeric_code: i32,
    pub minor_unit: i32,
    pub name: String,
}
