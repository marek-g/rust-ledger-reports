use chrono::NaiveDate;
use configuration::ReportParameters;
use configuration::VecDeref;
use date_utils::last_day_in_month;
use handlebars::to_json;
use input_data::*;
use ledger_utils::balance::Balance;
use ledger_utils::monthly_report::*;
use ledger_utils::prices::Prices;
use num_traits::cast::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use serde_json::value::{Map, Value as Json};

#[derive(Serialize)]
struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<TableCell>>,
}

enum TableCell {
    Month { year: i32, month: u32 },
    Value(Decimal),
}

impl TableCell {
    pub fn to_timestamp_millis(&self) -> Option<i64> {
        if let TableCell::Month { year, month } = self {
            Some(
                last_day_in_month(*year, *month)
                    .and_hms(0, 0, 0)
                    .timestamp_millis(),
            )
        } else {
            None
        }
    }

    pub fn to_value(&self) -> Option<Decimal> {
        if let TableCell::Value(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        if let Some(decimal) = self.to_value() {
            decimal.to_f64()
        } else {
            None
        }
    }
}

impl serde::Serialize for TableCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let text = match self {
            TableCell::Month { year, month } => format!("{}/{:02}", year, month),
            TableCell::Value(val) => format!("{}", val),
        };
        serializer.serialize_str(&text)
    }
}

#[derive(Serialize)]
struct Chart {
    id: String,
    min_x: f64,
    max_x: f64,
    digit_points: u32,
    series: String,
}

#[derive(Serialize)]
struct ChartSerie {
    key: String,
    values: Vec<[f64; 2]>,
}

pub fn make_report_data(
    input_data: &InputData,
    report_params: &ReportParameters,
) -> Map<String, Json> {
    let mut data = Map::new();

    configure_html_header(&mut data);

    let monthly_report = MonthlyReport::from(&input_data.ledger);

    let table_months = get_table_months(&monthly_report, &input_data.prices, &report_params);
    data.insert("table_months".to_string(), to_json(&table_months));

    let area_chart1 = get_area_chart1(&table_months);
    data.insert("area_chart1".to_string(), to_json(&area_chart1));

    data
}

fn configure_html_header(data: &mut Map<String, Json>) {
    let style = include_str!("templates/charts/nv.d3.css");

    let mut script = include_str!("templates/charts/d3.v3.js").to_owned();
    script.push_str("\n");
    script.push_str(include_str!("templates/charts/nv.d3.js"));

    data.insert("html_style".to_string(), to_json(style));
    data.insert("html_script".to_string(), to_json(script));
}

fn get_table_months(
    monthly_report: &MonthlyReport,
    prices: &Prices,
    params: &ReportParameters,
) -> Table {
    let headers = vec![
        "Date".to_string(),
        "Assets Sum".to_string(),
        "Assets Liquid".to_string(),
        "Assets Fixed".to_string(),
        "Assets High Risk".to_string(),
        "Income".to_string(),
        "Expenses".to_string(),
    ];

    let mut rows: Vec<Vec<TableCell>> = Vec::new();

    for monthly_balance in &monthly_report.monthly_balances {
        let last_day = last_day_in_month(monthly_balance.year, monthly_balance.month);

        let calc = MonthlyCalculator::new(&monthly_balance.total, &prices, last_day, &params);

        let assets_liquid = calc.get_value(&params.assets_liquid);
        let assets_fixed = calc.get_value(&params.assets_fixed);
        let assets_high_risk = calc.get_value(&params.assets_high_risk);
        let income = calc.get_value(&params.income);
        let expenses = calc.get_value(&params.expenses);

        rows.push(vec![
            TableCell::Month {
                year: monthly_balance.year,
                month: monthly_balance.month,
            },
            TableCell::Value(assets_liquid + assets_fixed + assets_high_risk),
            TableCell::Value(assets_liquid),
            TableCell::Value(assets_fixed),
            TableCell::Value(assets_high_risk),
            TableCell::Value(income),
            TableCell::Value(expenses),
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

    fn get_value(&self, accounts: &Vec<String>) -> Decimal {
        let assets_value = self
            .balance
            .get_account_balance(&(accounts.as_deref()))
            .value_in(&self.params.main_commodity, self.last_day, &self.prices);

        if let Ok(value) = assets_value {
            value.round_dp_with_strategy(
                self.params.main_commodity_decimal_points,
                RoundingStrategy::RoundHalfUp,
            )
        } else {
            Decimal::new(0, 0)
        }
    }
}

fn get_area_chart1(table_months: &Table) -> Chart {
    let min_date = table_months.rows[0][0].to_timestamp_millis().unwrap();
    let max_date = table_months.rows.last().unwrap()[0]
        .to_timestamp_millis()
        .unwrap();

    let mut series_assets_liquid = Vec::new();
    let mut series_assets_fixed = Vec::new();
    let mut series_assets_high_risk = Vec::new();
    for row in &table_months.rows {
        let date = row[0].to_timestamp_millis().unwrap() as f64;
        series_assets_liquid.push([date, row[2].to_f64().unwrap()]);
        series_assets_fixed.push([date, row[3].to_f64().unwrap()]);
        series_assets_high_risk.push([date, row[4].to_f64().unwrap()]);
    }

    Chart {
        id: "areaChart1".to_string(),
        min_x: min_date as f64,
        max_x: max_date as f64,
        digit_points: 0,
        series: to_json(vec![
            ChartSerie {
                key: "Liquid Assets".to_string(),
                values: series_assets_liquid,
            },
            ChartSerie {
                key: "Fixed Assets".to_string(),
                values: series_assets_fixed,
            },
            ChartSerie {
                key: "High Risk Assets".to_string(),
                values: series_assets_high_risk,
            },
        ]).to_string(),
    }
}
