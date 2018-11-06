pub struct Configuration {
    pub src_ledger_file: String,
    pub src_prices_file_opt: Option<String>,
    pub report_file: String,

    pub report_params: ReportParameters,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            src_ledger_file: "/mnt/truecrypt1/dokumenty/Finanse/ledger/marek.ledger".to_string(),
            src_prices_file_opt: Some(
                "/mnt/truecrypt1/dokumenty/Finanse/ledger/prices.db".to_string(),
            ),
            report_file: "/mnt/truecrypt1/dokumenty/Finanse/ledger/report.html".to_string(),
            report_params: ReportParameters::new(),
        }
    }
}

pub struct ReportParameters {
    pub main_commodity: String,
    pub main_commodity_decimal_points: u32,
    pub asset_account_prefixes: Vec<String>,
}

impl ReportParameters {
    pub fn new() -> ReportParameters {
        ReportParameters {
            main_commodity: "PLN".to_string(),
            main_commodity_decimal_points: 2,
            asset_account_prefixes: vec!["Aktywa:Płynne".to_string()],
        }
    }
}