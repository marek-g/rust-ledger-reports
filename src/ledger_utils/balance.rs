use chrono::NaiveDate;
use ledger_parser::*;
use crate::ledger_utils::prices::{Prices, PricesError};
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::collections::HashMap;
use std::fmt;
use std::ops::AddAssign;
use std::ops::SubAssign;

/// Balance of an account.
///
/// Maps commodity names to amounts.
#[derive(Clone)]
pub struct AccountBalance {
    pub amounts: HashMap<String, Amount>,
}

impl AccountBalance {
    pub fn new() -> AccountBalance {
        AccountBalance {
            amounts: HashMap::new(),
        }
    }

    pub fn value_in_commodity(
        &self,
        commodity_name: &str,
        date: NaiveDate,
        prices: &Prices,
    ) -> Result<Decimal, PricesError> {
        let mut result = Decimal::new(0, 0);
        for amount in self.amounts.values() {
            if amount.commodity.name == commodity_name {
                result += amount.quantity;
            } else {
                result += prices.convert(
                    amount.quantity,
                    &amount.commodity.name,
                    commodity_name,
                    date,
                )?;
            }
        }
        Ok(result)
    }

    pub fn value_in_commodity_rounded(
        &self,
        commodity_name: &str,
        decimal_points: u32,
        date: NaiveDate,
        prices: &Prices,
    ) -> Decimal {
        let assets_value = self.value_in_commodity(commodity_name, date, prices);
        if let Ok(value) = assets_value {
            value.round_dp_with_strategy(decimal_points, RoundingStrategy::RoundHalfUp)
        } else {
            Decimal::new(0, 0)
        }
    }

    fn remove_empties(&mut self) {
        let zero = Decimal::new(0, 0);
        let empties: Vec<String> = self
            .amounts
            .iter()
            .filter(|&(_, amount)| amount.quantity == zero)
            .map(|(k, _)| k.clone())
            .collect();
        for empty in empties {
            self.amounts.remove(&empty);
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
        self.remove_empties();
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
        self.remove_empties();
    }
}

impl fmt::Debug for AccountBalance {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut values: Vec<Amount> = self.amounts.values().cloned().collect();
        values.sort_by(|a, b| a.commodity.name.partial_cmp(&b.commodity.name).unwrap());
        write!(f, "{:?}", values)
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

            // TODO: handle empty amounts & balance verifications
            account_balance
                .amounts
                .entry(posting.amount.clone().unwrap().commodity.name)
                .and_modify(|a| a.quantity += posting.amount.clone().unwrap().quantity)
                .or_insert_with(|| posting.amount.clone().unwrap());
        }
    }

    pub fn get_account_balance(&self, account_prefixes: &[&str]) -> AccountBalance {
        let mut balance = AccountBalance::new();
        for (account_name, account_balance) in &self.account_balances {
            for account_prefix in account_prefixes {
                if account_name.starts_with(account_prefix) {
                    balance += account_balance;
                }
            }
        }
        balance
    }

    fn remove_empties(&mut self) {
        let empties: Vec<String> = self
            .account_balances
            .iter()
            .filter(|&(_, account_balance)| account_balance.amounts.len() == 0)
            .map(|(k, _)| k.clone())
            .collect();
        for empty in empties {
            self.account_balances.remove(&empty);
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
        self.remove_empties();
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
        self.remove_empties();
    }
}
