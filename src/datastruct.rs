use chrono::Utc;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum AccountType {
    Assets,
    Liabilities,
    Equities,
    Revenue,
    Expenses,
    Gains,
    Losses,
}
impl AccountType {
    pub fn from_i32(value: i32) -> AccountType {
        match value {
            0 => AccountType::Assets,
            1 => AccountType::Liabilities,
            2 => AccountType::Equities,
            3 => AccountType::Revenue,
            4 => AccountType::Expenses,
            5 => AccountType::Gains,
            6 => AccountType::Losses,
            _ => panic!("Unknown value: {}", value),
        }
    }
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
pub struct Account {
    pub id: i32,
    pub acc_type: AccountType,
    pub name: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub name: String,
    pub balance: f64,
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
    pub balance: f64,
    pub entry_type: EntryType,
}

#[derive(Debug, Serialize)]
pub struct SqlResult {
    pub value: f64,
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
