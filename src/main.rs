extern crate chrono;
extern crate handlebars;
extern crate ledger_parser;
extern crate rust_decimal;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod date_utils;
mod input_data;
mod ledger_utils;
mod report;
mod report_data;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let input_data = input_data::InputData::load(
        "/mnt/truecrypt1/dokumenty/Finanse/ledger/marek.ledger",
        Some("/mnt/truecrypt1/dokumenty/Finanse/ledger/prices.db"),
    )?;

    report::generate_report(
        "/mnt/truecrypt1/dokumenty/Finanse/ledger/report.html".to_string(),
        &input_data,
    )
}
