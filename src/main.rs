extern crate chrono;
extern crate handlebars;
extern crate ledger_parser;
extern crate rust_decimal;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod configuration;
mod date_utils;
mod input_data;
mod ledger_utils;
mod report;
mod report_data;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let configuration = configuration::Configuration::new();

    let input_data = input_data::InputData::load(
        &configuration.src_ledger_file,
        configuration
            .src_prices_file_opt
            .as_ref()
            .map(String::as_str),
    )?;

    report::generate_report(
        &configuration.report_file,
        &input_data,
        &configuration.report_params,
    )
}
