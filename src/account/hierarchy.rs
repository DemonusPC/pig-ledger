use crate::account::AccountAble;
use crate::account::AccountType;
use crate::account::AccountV2;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
            account: Option::None,
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
        AccountHierarchy::from_account_base(AccountType::Losses),
    ];
}

// Quite inefficient but it works
pub fn into_hierarchy(flat: Vec<AccountHierarchyStorage>) -> Vec<AccountHierarchy> {
    // println!("{:?}", flat);
    let mut root = root_hierarchy();

    let mut mem: HashMap<i32, Vec<AccountHierarchy>> = HashMap::new();

    for r in flat {
        let added = add_to_hierarchy(&mut root[r.acc_type as usize], &r, &mut mem);

        // If it hasn't been added it means that the flat hierarchy came out of order from its parent
        // Since it's parent doesn't exist yet it gets put into memory
        if !added {
            let out_of_order = match &r.leaf {
                true => AccountHierarchy::from_account(
                    r.parent,
                    AccountV2::new(
                        r.account_id.unwrap(),
                        r.acc_type,
                        String::from(r.acc_name.as_ref().unwrap()),
                        r.balance.unwrap(),
                        String::from(r.currency.as_ref().unwrap()),
                    ),
                ),
                false => AccountHierarchy::new(
                    r.h_id,
                    r.parent,
                    String::from(r.name.as_ref().unwrap()),
                    vec![],
                    Option::None,
                ),
            };

            if !mem.contains_key(&r.parent) {
                mem.insert(r.parent, vec![out_of_order]);
            } else {
                // Each item added should technically be unique so we store it in a vec
                mem.get_mut(&r.parent).unwrap().push(out_of_order);
            }
        }
    }

    // Any accounts that have not been added are added to the top level accounts
    for orphan in mem.into_iter().enumerate() {
        let hier = (orphan.1).1;
        for h in hier {
            if h.account.is_some() {
                &mut root[h.account.as_ref().unwrap().account_type() as usize].add_to_accounts(h);
            }
        }
    }
    
    root
}

// This doesn't account for hierarchies that are out of order
fn add_to_hierarchy(
    node: &mut AccountHierarchy,
    flat_account: &AccountHierarchyStorage,
    memory: &mut HashMap<i32, Vec<AccountHierarchy>>,
) -> bool {
    // We reached an end in this chain
    if node.account().is_some() {
        return false;
    }

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
                ));

                return true;
            }
        } else {
            
            // This doesn't deal with the orphans recursively. So some elements will not be added properly
            let next = match memory.contains_key(&flat_account.h_id) {
                true => {
                    let elements = memory.remove(&flat_account.h_id);
                    match elements{
                        Some(v) => v,
                        None => vec![]
                    }
                }
                false => vec![]
            };
            node.add_to_accounts(AccountHierarchy::new(
                flat_account.h_id,
                flat_account.parent,
                String::from(flat_account.name.as_ref().unwrap()),
                next,
                Option::None,
            ));


            return true;
        };
        return true;
    }


    for acc in node.accounts_mut() {
        let found = add_to_hierarchy(acc, flat_account, memory);
        if found {
            return true;
        }
    }

    return false;
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

    #[test]
    fn empty_hierarchy_produces_top_level_only(){
        let input : Vec<AccountHierarchyStorage> = vec![];

        let result = into_hierarchy(input);

        assert_eq!(result.len(), 7)

    }

    #[test]
    fn flat() {
        // Assets 
        // -> General 
        // Expenses
        // -> Tesco
        // Revenue
        // -> Salary

        let input : Vec<AccountHierarchyStorage> = vec![
            AccountHierarchyStorage::new(8, 0, Option::None , Option::from(8), AccountType::Assets, Option::from(String::from("General")), Option::from(10000), Option::from(String::from("GBP")), true),
            AccountHierarchyStorage::new(9, 4, Option::None , Option::from(9), AccountType::Expenses, Option::from(String::from("Tesco")), Option::from(500), Option::from(String::from("GBP")), true),
            AccountHierarchyStorage::new(10, 3, Option::None , Option::from(10), AccountType::Revenue, Option::from(String::from("Salary")), Option::from(0), Option::from(String::from("GBP")), true)
        ];

        let result = into_hierarchy(input);

        assert_eq!(result[0].accounts().len(), 1);
        assert_eq!(result[0].accounts()[0].account().as_ref().unwrap().id(), 8);
        assert_eq!(result[4].accounts().len(), 1);
        assert_eq!(result[4].accounts()[0].account().as_ref().unwrap().id(), 9);
        assert_eq!(result[3].accounts().len(), 1);
        assert_eq!(result[3].accounts()[0].account().as_ref().unwrap().id(), 10);

        assert_eq!(result[0].balance(), 10000);
        assert_eq!(result[4].balance(), 500);
        assert_eq!(result[3].balance(), 0);

    } 

    #[test]
    fn multi_layer() {
        // There are multiple layers. 
        // We always assume that top level hierarchies come first and leafs (accounts) come in last

        // Assets
        // -> Stocks
        //    -> NASDAQ
        //       -> TSLA
        //    -> FTSE
        //       -> OCDO
        // -> Current
        // Expenses
        // -> Food
        //    -> Chain
        //       -> Groceries
        //          -> Tesco

        let input : Vec<AccountHierarchyStorage> = vec![
            AccountHierarchyStorage::new(13, 4, Option::from(String::from("Food")) , Option::None, AccountType::Expenses, Option::None, Option::None, Option::None, false),
            AccountHierarchyStorage::new(7, 0, Option::from(String::from("Stocks")) , Option::None, AccountType::Assets, Option::None, Option::None, Option::None, false),
            AccountHierarchyStorage::new(8, 7, Option::from(String::from("NASDAQ")) , Option::None, AccountType::Assets, Option::None, Option::None, Option::None, false),
            AccountHierarchyStorage::new(14, 13, Option::from(String::from("Chain")) , Option::None, AccountType::Expenses, Option::None, Option::None, Option::None, false),
            AccountHierarchyStorage::new(15, 14, Option::from(String::from("Groceries")) , Option::None, AccountType::Expenses, Option::None, Option::None, Option::None, false),
            AccountHierarchyStorage::new(10, 7, Option::from(String::from("FTSE")) , Option::None, AccountType::Assets, Option::None, Option::None, Option::None, false),


            AccountHierarchyStorage::new(16, 8, Option::None , Option::from(9), AccountType::Assets, Option::from(String::from("Tesla Shares")), Option::from(5), Option::from(String::from("TSLA")), true),
            AccountHierarchyStorage::new(17, 10, Option::None , Option::from(11), AccountType::Assets, Option::from(String::from("Ocado Shares")), Option::from(1), Option::from(String::from("OCDO")), true),
            AccountHierarchyStorage::new(18, 0, Option::None , Option::from(12), AccountType::Assets, Option::from(String::from("Current Account")), Option::from(550000), Option::from(String::from("GPB")), true),
            AccountHierarchyStorage::new(19, 15, Option::None , Option::from(16), AccountType::Expenses, Option::from(String::from("Tesco")), Option::from(500), Option::from(String::from("GBP")), true),
        ];

        let result = into_hierarchy(input);
        
        // Assets have two nodes
        assert_eq!(result[0].accounts().len(), 2);
        // Expenses has one
        assert_eq!(result[4].accounts().len(), 1);

        println!("{:?}" , &result[4]);

        // Tesco is correctly chained
        let tesco = result[4].accounts()[0].accounts()[0].accounts()[0].accounts()[0].account().as_ref().unwrap();

        
        // Checking that the Tesco account that is deep in the chain is correctly placed
        assert_eq!(tesco.id(), 16); 
        assert_eq!(tesco.name(), "Tesco");
        assert_eq!(tesco.balance(), 500);
    }
}
