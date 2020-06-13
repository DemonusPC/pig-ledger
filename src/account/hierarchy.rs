use crate::account::AccountAble;
use crate::account::AccountType;
use crate::account::AccountV2;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountHierarchy {
    id: i32,
    // The parent account
    parent: i32,
    name: String,
    next: Vec<AccountHierarchy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<AccountV2>,
}

impl AccountHierarchy {
    pub fn new(
        id: i32,
        parent: i32,
        name: String,
        next: Vec<AccountHierarchy>,
        account: Option<AccountV2>,
    ) -> Self {
        AccountHierarchy {
            id,
            parent,
            name,
            next,
            account,
        }
    }

    pub fn from_account(parent: i32, account: AccountV2) -> Self {
        AccountHierarchy {
            id: account.id(),
            parent,
            name: String::from(account.name()),
            next: vec![],
            account: Option::from(account),
        }
    }

    pub fn from_account_base(account_type: AccountType) -> Self {
        AccountHierarchy {
            id: account_type as i32,
            parent: account_type as i32,
            name: account_type.into_string_identifier(),
            next: vec![],
            account: Option::None
        }
    }

    pub fn add_to_accounts(&mut self, item: AccountHierarchy) {
        self.next.push(item);
    }

    pub fn accounts(&self) -> &[AccountHierarchy] {
        &self.next
    }

    pub fn accounts_mut(&mut self) -> &mut Vec<AccountHierarchy> {
        &mut self.next
    }

    pub fn account(&self) -> &Option<AccountV2> {
        &self.account
    }

    pub fn account_mut(&mut self) -> &mut Option<AccountV2> {
        &mut self.account
    }
}

impl AccountAble for AccountHierarchy {
    fn name(&self) -> &str {
        &self.name
    }

    // Todo Implement Balance
    fn balance(&self) -> i32 {
        let mut balance = 0;

        match &self.account {
            Some(acc) => balance += acc.balance(),
            None => {
                for account in &self.next {
                    balance += account.balance();
                }
            }
        }
        balance
    }

    fn id(&self) -> i32 {
        self.id
    }
}

// Flat version that we get from the database
#[derive(Debug)]
pub struct AccountHierarchyStorage {
    pub h_id: i32,
    pub parent: i32,
    pub name: Option<String>,
    pub account_id: Option<i32>,
    pub acc_type: AccountType,
    pub acc_name: Option<String>,
    pub balance: Option<i32>,
    pub currency: Option<String>,
    pub leaf: bool,
}

impl AccountHierarchyStorage {
    pub fn new(
        h_id: i32,
        parent: i32,
        name: Option<String>,
        account_id: Option<i32>,
        acc_type: AccountType,
        acc_name: Option<String>,
        balance: Option<i32>,
        currency: Option<String>,
        leaf: bool,
    ) -> Self {
        AccountHierarchyStorage {
            h_id,
            parent,
            name,
            account_id,
            acc_type,
            acc_name,
            balance,
            currency,
            leaf,
        }
    }
}

fn root_hierarchy() -> Vec<AccountHierarchy> {
    return vec![
        AccountHierarchy::from_account_base(AccountType::Assets),
        AccountHierarchy::from_account_base(AccountType::Liabilities),
        AccountHierarchy::from_account_base(AccountType::Equities),
        AccountHierarchy::from_account_base(AccountType::Revenue),
        AccountHierarchy::from_account_base(AccountType::Expenses),
        AccountHierarchy::from_account_base(AccountType::Gains),
        AccountHierarchy::from_account_base(AccountType::Losses)
    ];
}

pub fn into_hierarchy(
    flat: Vec<AccountHierarchyStorage>
) -> Vec<AccountHierarchy> {
    
    let mut root = root_hierarchy();

    for r in flat {
        add_to_hierarchy(&mut root[r.acc_type as usize], &r);
    }
    root
}

// This doesn't account for hierarchies that are out of order
fn add_to_hierarchy(node: &mut AccountHierarchy, flat_account: &AccountHierarchyStorage) {
    if node.id() == flat_account.parent {
        if flat_account.leaf {
            if node.account().is_none() {
                node.add_to_accounts(AccountHierarchy::from_account(
                    flat_account.parent,
                    AccountV2::new(
                        flat_account.account_id.unwrap(),
                        flat_account.acc_type,
                        String::from(flat_account.acc_name.as_ref().unwrap()),
                        flat_account.balance.unwrap(),
                        String::from(flat_account.currency.as_ref().unwrap()),
                    ),
                ))
            }
        } else {
            node.add_to_accounts(AccountHierarchy::new(
                flat_account.h_id,
                flat_account.parent,
                String::from(flat_account.name.as_ref().unwrap()),
                vec![],
                Option::None,
            ))
        };
        return;
    }

    for acc in node.accounts_mut() {
        add_to_hierarchy(acc, flat_account)
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
            8,
            4,
            String::from("Groceries"),
            vec![
                AccountHierarchy::from_account(8, tesco),
                AccountHierarchy::from_account(8, sainsburys),
                AccountHierarchy::from_account(8, walmart),
                AccountHierarchy::from_account(8, amazon),
            ],
            Option::None,
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
            8,
            4,
            String::from("Cleaning"),
            vec![],
            Option::from(window_cleaning),
        );

        let repairs: AccountHierarchy = AccountHierarchy::new(
            9,
            4,
            String::from("Repairs"),
            vec![
                AccountHierarchy::from_account(9, plumbing),
                AccountHierarchy::from_account(9, electricity),
                AccountHierarchy::from_account(9, gas),
            ],
            Option::None,
        );

        let home: AccountHierarchy = AccountHierarchy::new(
            10,
            4,
            String::from("Groceries"),
            vec![cleaning, repairs],
            Option::None,
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
            9,
            4,
            String::from("Groceries"),
            vec![
                AccountHierarchy::from_account(9, walmart),
                AccountHierarchy::from_account(9, amazon),
            ],
            Option::None,
        );

        let food: AccountHierarchy = AccountHierarchy::new(
            8,
            4,
            String::from("Cleaning"),
            vec![groceries, AccountHierarchy::from_account(8, restaurant)],
            Option::None,
        );

        let result = food.balance();

        assert_eq!(result, 89899)
    }
}
