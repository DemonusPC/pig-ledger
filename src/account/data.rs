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

    pub fn into_string_identifier(&self) -> String {
        match self {
            AccountType::Assets => String::from("Assets"),
            AccountType::Liabilities => String::from("Liabilities"),
            AccountType::Equities => String::from("Equities"),
            AccountType::Revenue => String::from("Revenue"),
            AccountType::Expenses => String::from("Expenses"),
            AccountType::Gains => String::from("Gains"),
            AccountType::Losses => String::from("Losses"),
        }
    }
}

impl From<i32> for AccountType {
    fn from(number: i32) -> Self {
        match number {
            0 => AccountType::Assets,
            1 => AccountType::Liabilities,
            2 => AccountType::Equities,
            3 => AccountType::Revenue,
            4 => AccountType::Expenses,
            5 => AccountType::Gains,
            6 => AccountType::Losses,
            _ => panic!("Unknown value: {}", number),
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

impl Account {
    pub fn currency_compatible(&self, other: &Account) -> bool {
        if self.currency != other.currency {
            return false;
        }
        true
    }
}

#[derive(Debug, Serialize)]
pub struct DetailedAccount {
    pub id: i32,
    pub acc_type: AccountType,
    pub name: String,
    pub balance: i32,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccount {
    pub acc_type: i32,
    pub name: String,
    pub currency: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounttype_correctly_types() {
        let asset = AccountType::from_i32(0);
        let liabilities = AccountType::from_i32(1);
        let equities = AccountType::from_i32(2);
        let revenue = AccountType::from_i32(3);
        let expenses = AccountType::from_i32(4);
        let gains = AccountType::from_i32(5);
        let losses = AccountType::from_i32(6);

        assert_eq!(asset, AccountType::Assets);
        assert_eq!(liabilities, AccountType::Liabilities);
        assert_eq!(equities, AccountType::Equities);
        assert_eq!(revenue, AccountType::Revenue);
        assert_eq!(expenses, AccountType::Expenses);
        assert_eq!(gains, AccountType::Gains);
        assert_eq!(losses, AccountType::Losses)
    }
}
