use ledger_parser::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AccountBalance {
    /// maps currency name to amount
    pub amounts: HashMap<String, Amount>,
}

impl AccountBalance {
    pub fn new() -> AccountBalance {
        AccountBalance {
            amounts: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Balance {
    // maps account name to account balance
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
