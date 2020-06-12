use crate::account::AccountAble;
use crate::account::AccountType;
// use serde_derive::{Deserialize, Serialize};

// #[derive(Serialize)]
pub struct AccountHierarchy {
    // The parent account
    parent: i32,
    name: String,
    accounts: Vec<Box<dyn AccountAble>>,
}

impl AccountHierarchy {
    pub fn new(parent: i32, name: String, accounts: Vec<Box<dyn AccountAble>>) -> Self {
        AccountHierarchy {
            parent,
            name,
            accounts,
        }
    }
}

impl AccountAble for AccountHierarchy {
    fn name(&self) -> &str {
        &self.name
    }

    // Todo Implement Balance
    fn balance(&self) -> i32 {
        let mut balance = 0;

        for account in &self.accounts {
            balance += account.balance();
        }
        balance

    }
}

// Flat version that we get from the database
pub struct AccountHierarchyStorage {
    pub h_id: i32,
    pub parent: i32,
    pub name: Option<String>,
    pub acc_type: Option<AccountType>,
    pub balance: Option<i32>, 
    pub currency: Option<String>,
    pub leaf: bool
}

impl AccountHierarchyStorage {
    pub fn new(h_id: i32, parent: i32, name: Option<String>, acc_type: Option<AccountType>, balance: Option<i32>, currency: Option<String>, leaf: bool) -> Self {
        AccountHierarchyStorage {
            h_id,
            parent,
            name,
            acc_type,
            balance,
            currency,
            leaf
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::AccountType;
    use crate::account::AccountV2;

    #[test]
    fn balance_of_a_single_level() {
        let tesco = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Tesco"),
            1000,
            String::from("GBP"),
        );
        let sainsburys = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Sainsburys"),
            25800,
            String::from("GBP"),
        );
        let walmart = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Walmart"),
            150,
            String::from("GBP"),
        );
        let amazon = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Amazon Groceries"),
            72300,
            String::from("GBP"),
        );

        let groceries: AccountHierarchy = AccountHierarchy::new(
            4,
            String::from("Groceries"),
            vec![
                Box::new(tesco),
                Box::new(sainsburys),
                Box::new(walmart),
                Box::new(amazon),
            ],
        );

        let result = groceries.balance();

        assert_eq!(result, 99250)
    }

    #[test]
    fn balance_of_two_levels() {
        let window_cleaning = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Window Cleaning"),
            10000,
            String::from("GBP"),
        );

        let plumbing = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Plumbing"),
            0,
            String::from("GBP"),
        );
        let electricity = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Electricity"),
            3200,
            String::from("GBP"),
        );
        let gas = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Gas"),
            51,
            String::from("GBP"),
        );

        
        let cleaning: AccountHierarchy = AccountHierarchy::new(
            4,
            String::from("Cleaning"),
            vec![
                Box::new(window_cleaning),
            ],
        );

        let repairs: AccountHierarchy = AccountHierarchy::new(
            4,
            String::from("Repairs"),
            vec![
                Box::new(plumbing),
                Box::new(electricity),
                Box::new(gas),
            ],
        );

        let home: AccountHierarchy = AccountHierarchy::new(
            4,
            String::from("Groceries"),
            vec![
                Box::new(cleaning),
                Box::new(repairs),
            ],
        );


        let result = home.balance();

        assert_eq!(result, 13251)
    }

    #[test]
    fn balance_of_two_different_levels() {
        let restaurant = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Restaurant"),
            25000,
            String::from("GBP"),
        );

        let walmart = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Walmart"),
            5000,
            String::from("GBP"),
        );
        let amazon = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Amazon Groceries"),
            59899,
            String::from("GBP"),
        );


        


        let groceries: AccountHierarchy = AccountHierarchy::new(
            4,
            String::from("Groceries"),
            vec![
                Box::new(walmart),
                Box::new(amazon),
            ],
        );

        let food: AccountHierarchy = AccountHierarchy::new(
            4,
            String::from("Cleaning"),
            vec![
                Box::new(groceries),
                Box::new(restaurant),
            ],
        );


        let result = food.balance();

        assert_eq!(result, 89899)
    }
}
