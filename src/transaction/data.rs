use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: i32,
    pub date: chrono::DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: i32,
    pub account: i32,
    pub account_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransaction {
    pub name: String,
    pub balance: i32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EntryV2 {
    id: i32,
    account: i32,
    account_name: String,
    transaction_id: i32,
    balance: i32,
    entry_type: EntryType,
}

impl EntryV2 {
    pub fn new(
        id: i32,
        account: i32,
        account_name: String,
        transaction_id: i32,
        balance: i32,
        entry_type: EntryType,
    ) -> EntryV2 {
        EntryV2 {
            id: id,
            account: account,
            account_name: account_name,
            transaction_id: transaction_id,
            balance: balance,
            entry_type: entry_type,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn account(&self) -> i32 {
        self.account
    }

    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    pub fn transaction_id(&self) -> i32 {
        self.transaction_id
    }

    pub fn balance(&self) -> i32 {
        self.balance
    }

    pub fn entry_type(&self) -> EntryType {
        self.entry_type
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionV2 {
    id: i32,
    date: chrono::DateTime<Utc>,
    name: String,
    entries: Vec<EntryV2>,
}

impl TransactionV2 {
    pub fn new(
        id: i32,
        date: chrono::DateTime<Utc>,
        name: String,
        entries: Vec<EntryV2>,
    ) -> TransactionV2 {
        TransactionV2 {
            id: id,
            date: date,
            name: name,
            entries: entries,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn date(&self) -> chrono::DateTime<Utc> {
        self.date
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn entries(&self) -> &[EntryV2] {
        &self.entries
    }
}

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    year: Option<i32>,
    month: Option<u8>,
}

impl DateQuery {
    pub fn year(&self) -> Option<i32> {
        self.year
    }
    pub fn month(&self) -> Option<u8> {
        self.month
    }

    // Returns if the query has a valid date
    // Returns true if query is empty
    pub fn valid_date(&self) -> bool {
        let month: bool = match self.month {
            Some(v) => {
                if v < 1 || v > 12 {
                    return false;
                }
                true
            }
            None => true,
        };

        let year: bool = match self.year {
            Some(v) => {
                if v < 1970 {
                    return false;
                }
                true
            }
            None => true,
        };

        if month && year {
            return true;
        }

        false
    }

    pub fn only_year(&self) -> bool {
        if self.year.is_some() && self.month.is_none() {
            return true;
        }

        false
    }

    pub fn only_month(&self) -> bool {
        if self.year.is_none() && self.month.is_some() {
            return true;
        }

        false
    }

    pub fn is_full(&self) -> bool {
        if self.year.is_some() && self.month.is_some() {
            return true;
        }

        false
    }
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
