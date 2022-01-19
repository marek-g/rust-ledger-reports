use ledger_parser::*;
use ledger_utils::join_ledgers::join_ledgers;
use ledger_utils::prices::Prices;
use std::error::Error;

pub struct InputData {
    pub ledger: Ledger,
    pub prices: Prices,
}

impl InputData {
    pub fn load(ledger_files: &Vec<String>) -> Result<InputData, Box<dyn Error>> {
        let ledgers: Result<Vec<Ledger>, Box<dyn Error>> = ledger_files
            .iter()
            .map(|file_name| Ok(parse(&std::fs::read_to_string(file_name)?)?))
            .collect();

        let ledger = join_ledgers(ledgers?);
        let prices = Prices::load(&ledger);

        Ok(InputData { ledger, prices })
    }
}
