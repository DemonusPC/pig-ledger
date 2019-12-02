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

#[derive(Debug, Serialize)]
pub struct Account {
    pub id: i32,
    pub acc_type: AccountType,
    pub name: String,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct DetailedAccount {
    pub id: i32,
    pub acc_type: AccountType,
    pub name: String,
    pub balance: i32,
    pub currency: String,
}
