use ledger_parser::*;
use std::error::Error;

pub struct InputData {
    pub ledger: Ledger,
}

impl InputData {
    pub fn load(ledger_file: &str) -> Result<InputData, Box<Error>> {
        let file_content =
            std::fs::read_to_string("/mnt/truecrypt1/dokumenty/Finanse/ledger/marek.ledger")?;
        //std::fs::read_to_string("/mnt/truecrypt1/dokumenty/Finanse/ledger/prices.db").unwrap();

        let ledger = parse(&file_content)?;

        Ok(InputData { ledger: ledger })
    }
}
