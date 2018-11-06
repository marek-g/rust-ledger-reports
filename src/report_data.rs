use chrono::NaiveDate;
use configuration::ReportParameters;
use configuration::VecDeref;
use date_utils::last_day_in_month;
use handlebars::to_json;
use input_data::*;
use ledger_utils::balance::Balance;
use ledger_utils::monthly_report::*;
use ledger_utils::prices::Prices;
use rust_decimal::RoundingStrategy;
use serde_json::value::{Map, Value as Json};

#[derive(Serialize)]
struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

pub fn make_report_data(
    input_data: &InputData,
    report_params: &ReportParameters,
) -> Map<String, Json> {
    let mut data = Map::new();

    let monthly_report = MonthlyReport::from(&input_data.ledger);

    let table_months = get_table_months(&monthly_report, &input_data.prices, &report_params);
    data.insert("table_months".to_string(), to_json(&table_months));

    data
}

fn get_table_months(
    monthly_report: &MonthlyReport,
    prices: &Prices,
    params: &ReportParameters,
) -> Table {
    let headers = vec![
        "Date".to_string(),
        "Assets Liquid".to_string(),
        "Assets Fixed".to_string(),
        "Assets High Risk".to_string(),
        "Income".to_string(),
        "Expenses".to_string(),
    ];

    let mut rows: Vec<Vec<String>> = Vec::new();

    for monthly_balance in &monthly_report.monthly_balances {
        // date
        let date = format!("{}/{:02}", monthly_balance.year, monthly_balance.month);
        let last_day = last_day_in_month(monthly_balance.year, monthly_balance.month);

        let calc = MonthlyCalculator::new(&monthly_balance.total, &prices, last_day, &params);

        let assets_liquid = calc.get_value(&params.assets_liquid);
        let assets_fixed = calc.get_value(&params.assets_fixed);
        let assets_high_risk = calc.get_value(&params.assets_high_risk);
        let income = calc.get_value(&params.income);
        let expenses = calc.get_value(&params.expenses);

        rows.push(vec![
            date,
            assets_liquid,
            assets_fixed,
            assets_high_risk,
            income,
            expenses,
        ]);
    }

    Table {
        headers: headers,
        rows: rows,
    }
}

struct MonthlyCalculator<'a> {
    balance: &'a Balance,
    prices: &'a Prices,
    last_day: NaiveDate,
    params: &'a ReportParameters,
}

impl<'a> MonthlyCalculator<'a> {
    pub fn new(
        balance: &'a Balance,
        prices: &'a Prices,
        last_day: NaiveDate,
        params: &'a ReportParameters,
    ) -> MonthlyCalculator<'a> {
        MonthlyCalculator {
            balance: balance,
            prices: prices,
            last_day: last_day,
            params: params,
        }
    }

    fn get_value(&self, accounts: &Vec<String>) -> String {
        let assets_value = self
            .balance
            .get_account_balance(&(accounts.as_deref()))
            .value_in(&self.params.main_commodity, self.last_day, &self.prices);

        if let Ok(value) = assets_value {
            format!(
                "{} {}",
                value.round_dp_with_strategy(
                    self.params.main_commodity_decimal_points,
                    RoundingStrategy::RoundHalfUp
                ),
                self.params.main_commodity
            )
        } else {
            format!("? {}", self.params.main_commodity)
        }
    }
}
