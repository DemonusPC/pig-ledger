use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub open: chrono::DateTime<Utc>,
    pub close: chrono::DateTime<Utc>,
    #[serde(skip)]
    target: String,
}

impl Budget {
    pub fn new(
        id: i32,
        name: &String,
        open: chrono::DateTime<Utc>,
        close: chrono::DateTime<Utc>,
        target: &String,
    ) -> Budget {
        Budget {
            id: id,
            name: String::from(name),
            open: open,
            close: close,
            target: String::from(target),
        }
    }

    pub fn get_target(&self) -> &String {
        return &self.target;
    }
}

#[derive(Debug, Deserialize)]
pub struct NewBudget {
    pub name: String,
    pub open: String,
    pub close: String,
}

#[derive(Debug, Deserialize)]
pub struct BudgetEntry {
    pub id: i32,
    pub account: i32,
    pub budget: i32,
    pub balance: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewBudgetEntry {
    pub account: i32,
    pub budget: i32,
    pub balance: i32,
}


