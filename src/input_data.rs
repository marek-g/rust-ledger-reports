use ledger_parser::*;
use crate::ledger_utils::prices::Prices;
use std::error::Error;

pub struct InputData {
    pub ledger: Ledger,
    pub prices: Prices,
}

impl InputData {
    pub fn load(ledger_file: &str, prices_file: Option<&str>) -> Result<InputData, Box<dyn Error>> {
        let file_content = std::fs::read_to_string(ledger_file)?;
        let ledger = parse(&file_content)?;

        let prices = if let Some(prices_file) = prices_file {
            let file_content = std::fs::read_to_string(prices_file)?;
            let prices_ledger = parse(&file_content)?;
            Prices::load(&ledger, Some(&prices_ledger))
        } else {
            Prices::load(&ledger, None)
        };

        Ok(InputData {
            ledger: ledger,
            prices: prices,
        })
    }
}
