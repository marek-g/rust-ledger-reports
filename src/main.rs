extern crate chrono;
extern crate handlebars;
extern crate ledger_parser;
extern crate num_traits;
extern crate rust_decimal;
extern crate serde;
extern crate serde_json;

mod configuration;
mod date_utils;
mod input_data;
mod ledger_utils;
mod report;
mod report_data;

use clap::{App, Arg, ArgMatches};
use std::error::Error;

use crate::configuration::Configuration;
use crate::ledger_utils::handle_foreign_currencies::handle_foreign_currencies;
use ledger_parser::{Serializer, SerializerSettings};

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

    let configuration = read_configuration(&matches)?;

    let mut input_data = input_data::InputData::load(&configuration.ledger_files)?;

    let is_asset_account = |account_name: &str| {
        for asset_prefix in &configuration.report_params.assets {
            if account_name.starts_with(asset_prefix) {
                return true;
            }
        }
        false
    };

    let is_income_account = |account_name: &str| {
        for income_prefix in &configuration.report_params.income {
            if account_name.starts_with(income_prefix) {
                return true;
            }
        }
        false
    };

    let is_expense_account = |account_name: &str| {
        for expense_prefix in &configuration.report_params.expenses {
            if account_name.starts_with(expense_prefix) {
                return true;
            }
        }
        false
    };

    if let Err(err) = handle_foreign_currencies(
        &mut input_data.ledger,
        &is_asset_account,
        &is_income_account,
        &is_expense_account,
        &configuration.report_params.main_commodity,
        configuration.report_params.main_commodity_decimal_points,
        &input_data.prices,
    ) {
        panic!("{:?}", err);
    }

    println!(
        "{}",
        input_data
            .ledger
            .to_string_pretty(&SerializerSettings::with_indent("\t"))
    );

    report::generate_report(
        &configuration.report_file,
        &input_data.ledger,
        &input_data.prices,
        &configuration.report_params,
    )
}

fn read_configuration(matches: &ArgMatches) -> Result<Configuration, Box<dyn Error>> {
    let config_file_name = matches
        .value_of("config")
        .map(|c| c.to_string())
        .unwrap_or_else(|| {
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("ledger_reports.toml")
                .to_str()
                .unwrap()
                .to_string()
        });
    let config_content = std::fs::read_to_string(config_file_name)?;
    let configuration: Configuration = toml::from_str(&config_content)?;
    Ok(configuration)
}
