use ledger_parser::*;
use ledger_utils::join_ledgers::join_ledgers;
use ledger_utils::prices::Prices;
use ledger_utils::{simplified_ledger, SimplificationError};
use std::convert::TryFrom;
use std::error::Error;

pub struct InputData {
    pub ledger: simplified_ledger::Ledger,
    pub prices: Prices,
}

impl InputData {
    pub fn load(ledger_files: &Vec<String>) -> Result<InputData, Box<dyn Error>> {
        let ledgers: Result<Vec<Ledger>, Box<dyn Error>> = ledger_files
            .iter()
            .map(|file_name| Ok(parse(&std::fs::read_to_string(file_name)?)?))
            .collect();
        let ledgers = ledgers?;

        let mut prices = Prices::new();
        for ledger in &ledgers {
            prices.insert_from(ledger);
        }

        let simplified_ledgers: Result<Vec<simplified_ledger::Ledger>, SimplificationError> =
            ledgers
                .into_iter()
                .map(|ledger| simplified_ledger::Ledger::try_from(ledger))
                .collect();

        let simplified_ledger = join_ledgers(simplified_ledgers?);

        Ok(InputData {
            ledger: simplified_ledger,
            prices,
        })
    }
}
