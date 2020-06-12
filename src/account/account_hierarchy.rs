use crate::account::AccountAble;
use crate::account::AccountV2;

struct AccountHierarchy {
    // The parent account
    parent: i32,
    name: String,
    accounts: Vec<Box<AccountAble>>,
}

impl AccountHierarchy {
    pub fn new(parent: i32, name: String, accounts: Vec<Box<AccountAble>>) -> Self {
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
}
