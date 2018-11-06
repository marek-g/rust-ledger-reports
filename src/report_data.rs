use configuration::ReportParameters;
use configuration::VecDeref;
use date_utils::last_day_in_month;
use handlebars::to_json;
use input_data::*;
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
    let headers = vec!["Date".to_string(), "Assets".to_string()];

    let mut rows: Vec<Vec<String>> = Vec::new();

    for monthly_balance in &monthly_report.monthly_balances {
        // date
        let date = format!("{}/{:02}", monthly_balance.year, monthly_balance.month);
        let last_day = last_day_in_month(monthly_balance.year, monthly_balance.month);

        let assets_value = monthly_balance
            .total
            .get_account_balance(&(params.asset_account_prefixes.as_deref()))
            .value_in(&params.main_commodity, last_day, &prices);
        let assets = if let Ok(value) = assets_value {
            format!(
                "{} {}",
                value.round_dp_with_strategy(
                    params.main_commodity_decimal_points,
                    RoundingStrategy::RoundHalfUp
                ),
                params.main_commodity
            )
        } else {
            format!("? {}", params.main_commodity)
        };

        rows.push(vec![date, assets]);
    }

    Table {
        headers: headers,
        rows: rows,
    }
}
