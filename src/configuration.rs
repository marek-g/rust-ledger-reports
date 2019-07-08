pub struct Configuration {
    pub src_ledger_file: String,
    pub src_prices_file_opt: Option<String>,
    pub report_file: String,

    pub report_params: ReportParameters,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            src_ledger_file: "/media/truecrypt1/dokumenty/Finanse/ledger/marek.ledger".to_string(),
            src_prices_file_opt: Some(
                "/media/truecrypt1/dokumenty/Finanse/ledger/prices.db".to_string(),
            ),
            report_file: "/media/truecrypt1/dokumenty/Finanse/ledger/report.html".to_string(),
            report_params: ReportParameters::new(),
        }
    }
}

pub struct ReportParameters {
    pub main_commodity: String,
    pub main_commodity_decimal_points: u32,

    pub assets_liquid: Vec<String>,
    pub assets_fixed: Vec<String>,
    pub assets_high_risk: Vec<String>,

    pub income: Vec<String>,

    pub expenses: Vec<String>,
}

impl ReportParameters {
    pub fn new() -> ReportParameters {
        ReportParameters {
            main_commodity: "PLN".to_string(),
            main_commodity_decimal_points: 2,

            assets_liquid: vec!["Aktywa:Płynne".to_string()],
            assets_fixed: vec!["Aktywa:Stałe".to_string()],
            assets_high_risk: vec!["Aktywa:Kryptowaluty".to_string()],

            income: vec!["Przychody".to_string()],

            expenses: vec!["Wydatki".to_string()],
        }
    }
}

use std::ops::Deref;

pub trait OptionDeref<T: Deref> {
    fn as_deref(&self) -> Option<&T::Target>;
}

impl<T: Deref> OptionDeref<T> for Option<T> {
    fn as_deref(&self) -> Option<&T::Target> {
        self.as_ref().map(Deref::deref)
    }
}

pub trait VecDeref<T: Deref> {
    fn as_deref(&self) -> Vec<&T::Target>;
}

impl<T: Deref> VecDeref<T> for Vec<T> {
    fn as_deref(&self) -> Vec<&T::Target> {
        self.iter().map(Deref::deref).collect::<Vec<&T::Target>>()
    }
}
