use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub date: chrono::DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum EntryType {
    Debit,
    Credit,
}

impl EntryType {
    pub fn from_i32(value: i32) -> EntryType {
        match value {
            0 => EntryType::Credit,
            1 => EntryType::Debit,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: i32,
    pub account: i32,
    pub transaction_id: i32,
    pub balance: i32,
    pub entry_type: EntryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub name: String,
    pub balance: i32,
    pub from: i32,
    pub to: i32,
}
