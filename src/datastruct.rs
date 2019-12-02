use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

use crate::account::data::AccountType;

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
pub struct Account {
    pub id: i32,
    pub acc_type: AccountType,
    pub name: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub name: String,
    pub balance: i32,
    pub from: i32,
    pub to: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccount {
    pub acc_type: i32,
    pub name: String,
    pub currency: String,
}

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
pub struct Entry {
    pub id: i32,
    pub account: i32,
    pub transaction_id: i32,
    pub balance: i32,
    pub entry_type: EntryType,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_correctly_types() {
        let debit = EntryType::from_i32(1);
        let credit = EntryType::from_i32(0);

        assert_eq!(debit, EntryType::Debit);
        assert_eq!(credit, EntryType::Credit)
    }
}
