use chrono::Datelike;
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
}

#[derive(Debug, Clone)]
pub struct MonthlyBalance {
    pub year: i32,
    pub month: u32,
    pub monthly_change: Balance,
    pub total: Balance,
}

impl MonthlyBalance {
    pub fn new(year: i32, month: u32) -> MonthlyBalance {
        MonthlyBalance {
            year: year,
            month: month,
            monthly_change: Balance::new(),
            total: Balance::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonthlyReport {
    pub monthly_balances: Vec<MonthlyBalance>,
}

impl MonthlyReport {
    pub fn new() -> MonthlyReport {
        MonthlyReport {
            monthly_balances: Vec::new(),
        }
    }
}

pub fn get_balance(ledger: &Ledger) -> Balance {
    let mut balance = Balance::new();

    for transaction in &ledger.transactions {
        update_balance_with_transaction(&mut balance, transaction);
    }

    balance
}

pub fn get_monthly_report(ledger: &Ledger) -> MonthlyReport {
    let mut report = MonthlyReport::new();

    let mut current_year = 0;
    let mut current_month = 0;
    let mut current_montly_balance: Option<MonthlyBalance> = None;
    let mut monthly_balance = Balance::new();
    let mut total_balance = Balance::new();

    for transaction in &ledger.transactions {
        if transaction.date.year() != current_year || transaction.date.month() != current_month {
            // begin new month

            if let Some(mut b) = current_montly_balance.take() {
                b.monthly_change = monthly_balance.clone();
                b.total = total_balance.clone();
                report.monthly_balances.push(b);
            }

            current_year = transaction.date.year();
            current_month = transaction.date.month();
            monthly_balance = Balance::new();

            current_montly_balance = Some(MonthlyBalance::new(current_year, current_month));
        }

        update_balance_with_transaction(&mut monthly_balance, transaction);
        update_balance_with_transaction(&mut total_balance, transaction);
    }

    if let Some(monthly_balance) = current_montly_balance.take() {
        report.monthly_balances.push(monthly_balance);
    }

    report
}

fn update_balance_with_transaction(balance: &mut Balance, transaction: &Transaction) {
    for posting in &transaction.postings {
        let account_balance = balance
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
