use crate::account::AccountV2;
use crate::account::AccountAble;

struct AccountHierarchy {
    name: String,
    accounts: Vec<AccountV2>
}

impl AccountHierarchy {
    pub fn new(name: String, accounts: Vec<AccountV2>) -> Self {
        AccountHierarchy {
            name, 
            accounts
        }
    }
}

impl AccountAble for AccountHierarchy {
    fn name(&self) -> &str {
        &self.name
    }

    // Todo Implement Balance
    fn balance(&self) -> i32 {
        100
    } 
}
