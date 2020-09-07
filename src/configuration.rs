use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub ledger_files: Vec<String>,
    pub report_file: String,

    pub report_params: ReportParameters,
}

#[derive(Deserialize)]
pub struct ReportParameters {
    pub main_commodity: String,
    pub main_commodity_decimal_points: u32,

    pub assets_liquid: Vec<String>,
    pub assets_fixed: Vec<String>,
    pub assets_high_risk: Vec<String>,

    pub total_income: Vec<String>,
    pub job_income: Vec<String>,
    pub investment_income: Vec<String>,

    pub expenses: Vec<String>,
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
