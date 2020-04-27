extern crate chrono;
extern crate handlebars;
extern crate ledger_parser;
extern crate rust_decimal;
extern crate serde;
extern crate num_traits;
extern crate serde_json;

mod configuration;
mod date_utils;
mod input_data;
mod ledger_utils;
mod report;
mod report_data;

use clap::{App, Arg};
use std::error::Error;

use crate::configuration::Configuration;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rust_reports")
        .version("0.1")
        .about("Converts ledger-cli file to html-report.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file (toml)")
                .takes_value(true),
        )
        .get_matches();

    let config_file_name = matches.value_of("config").map(|c| c.to_string()).unwrap_or_else(|| {
        std::env::current_exe().unwrap().parent().unwrap().join("ledger_reports.toml").to_str().unwrap().to_string()
    });
    let config_content = std::fs::read_to_string(config_file_name)?;
    let configuration: Configuration = toml::from_str(&config_content)?;

    let input_data = input_data::InputData::load(
        &configuration.src_ledger_file,
        configuration.src_prices_file_opt.as_deref(),
    )?;

    report::generate_report(
        &configuration.report_file,
        &input_data,
        &configuration.report_params,
    )
}
