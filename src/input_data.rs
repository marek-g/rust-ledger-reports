use ledger_parser::*;
use ledger_utils::prices::Prices;
use std::error::Error;

pub struct InputData {
    pub ledger: Ledger,
    pub prices: Prices,
}

impl InputData {
    pub fn load(ledger_file: &str, prices_file: &str) -> Result<InputData, Box<Error>> {
        let file_content = std::fs::read_to_string(ledger_file)?;
        let ledger = parse(&file_content)?;

        let file_content = std::fs::read_to_string(prices_file)?;
        let prices_ledger = parse(&file_content)?;
        let prices = Prices::load(&ledger, Some(&prices_ledger));

        Ok(InputData {
            ledger: ledger,
            prices: prices,
        })
    }
}
