use serde_derive::{Deserialize, Serialize};

use crate::account::{AccountAble, AccountType};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountV2 {
    id: i32,
    acc_type: AccountType,
    name: String,
    balance: i32,
    currency: String,
}

impl AccountAble for AccountV2 {
    fn balance(&self) -> i32 {
        self.balance
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl AccountV2 {
    pub fn new(
        id: i32,
        acc_type: AccountType,
        name: String,
        balance: i32,
        currency: String,
    ) -> Self {
        AccountV2 {
            id,
            acc_type,
            name,
            balance,
            currency,
        }
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    // Accounts are compatible if their currencies match
    pub fn compatible(&self, other: &AccountV2) -> bool {
        if self.currency() == other.currency {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_currencies_are_compatible() {
        let acc1 = AccountV2::new(
            1,
            AccountType::from_i32(0),
            String::from("Current Account"),
            1000,
            String::from("GBP"),
        );
        let acc2 = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Groceries"),
            1000,
            String::from("GBP"),
        );

        let result = acc1.compatible(&acc2);

        assert_eq!(result, true)
    }

    #[test]
    fn different_currencies_dont_match_accounts() {
        let acc1 = AccountV2::new(
            1,
            AccountType::from_i32(0),
            String::from("Current Account"),
            1000,
            String::from("GBP"),
        );
        let acc2 = AccountV2::new(
            2,
            AccountType::from_i32(4),
            String::from("Tesla Stock"),
            1000,
            String::from("TSLA"),
        );

        let result = acc1.compatible(&acc2);

        assert_eq!(result, false)
    }
}
