use ledger_parser::*;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::ops::SubAssign;

/// Balance of an account.
///
/// Maps currency names to amounts.
#[derive(Debug, Clone)]
pub struct AccountBalance {
    pub amounts: HashMap<String, Amount>,
}

impl AccountBalance {
    pub fn new() -> AccountBalance {
        AccountBalance {
            amounts: HashMap::new(),
        }
    }
}

impl<'a> AddAssign<&'a AccountBalance> for AccountBalance {
    fn add_assign(&mut self, other: &'a AccountBalance) {
        for (currrency_name, amount) in &other.amounts {
            self.amounts
                .entry(currrency_name.clone())
                .and_modify(|a| a.quantity += amount.quantity)
                .or_insert_with(|| amount.clone());
        }
    }
}

impl<'a> SubAssign<&'a AccountBalance> for AccountBalance {
    fn sub_assign(&mut self, other: &'a AccountBalance) {
        for (currrency_name, amount) in &other.amounts {
            self.amounts
                .entry(currrency_name.clone())
                .and_modify(|a| a.quantity -= amount.quantity)
                .or_insert_with(|| amount.clone());
        }
    }
}

/// Balance of one or more accounts.
///
/// Maps account names to their balances.
#[derive(Debug, Clone)]
pub struct Balance {
    pub account_balances: HashMap<String, AccountBalance>,
}

impl Balance {
    pub fn new() -> Balance {
        Balance {
            account_balances: HashMap::new(),
        }
    }

    pub fn update_with_transaction(&mut self, transaction: &Transaction) {
        for posting in &transaction.postings {
            let account_balance = self
                .account_balances
                .entry(posting.account.clone())
                .or_insert_with(|| AccountBalance::new());

            account_balance
                .amounts
                .entry(posting.amount.commodity.name.clone())
                .and_modify(|a| a.quantity += posting.amount.quantity)
                .or_insert_with(|| posting.amount.clone());
        }
    }
}

impl<'a> From<&'a Ledger> for Balance {
    fn from(ledger: &'a Ledger) -> Self {
        let mut balance = Balance::new();

        for transaction in &ledger.transactions {
            balance.update_with_transaction(transaction);
        }

        balance
    }
}

impl<'a> From<&'a Transaction> for Balance {
    fn from(transaction: &'a Transaction) -> Self {
        let mut balance = Balance::new();
        balance.update_with_transaction(transaction);
        balance
    }
}

impl<'a> AddAssign<&'a Balance> for Balance {
    fn add_assign(&mut self, other: &'a Balance) {
        for (account_name, account_balance) in &other.account_balances {
            self.account_balances
                .entry(account_name.clone())
                .and_modify(|b| *b += account_balance)
                .or_insert_with(|| account_balance.clone());
        }
    }
}

impl<'a> SubAssign<&'a Balance> for Balance {
    fn sub_assign(&mut self, other: &'a Balance) {
        for (account_name, account_balance) in &other.account_balances {
            self.account_balances
                .entry(account_name.clone())
                .and_modify(|b| *b -= account_balance)
                .or_insert_with(|| account_balance.clone());
        }
    }
}
