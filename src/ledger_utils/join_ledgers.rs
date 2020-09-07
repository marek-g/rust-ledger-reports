use ledger_parser::Ledger;

pub fn join_ledgers(ledgers: Vec<Ledger>) -> Ledger {
    let mut ledger = Ledger {
        transactions: Vec::new(),
        commodity_prices: Vec::new(),
    };

    for mut src_ledger in ledgers {
        ledger.commodity_prices.append(&mut src_ledger.commodity_prices);
        ledger.transactions.append(&mut src_ledger.transactions);
    }

    ledger
}
