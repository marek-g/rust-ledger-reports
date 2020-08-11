use chrono::prelude::*;
use crate::configuration::{VecDeref, ReportParameters};
use crate::date_utils::last_day_in_month;
use handlebars::to_json;
use crate::input_data::*;
use crate::ledger_utils::balance::Balance;
use crate::ledger_utils::monthly_report::*;
use crate::ledger_utils::prices::Prices;
use num_traits::cast::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use serde::Serialize;
use serde_json::value::{Map, Value as Json};
use crate::ledger_utils::tree_balance::TreeBalanceNode;
use rust_decimal::prelude::Zero;

#[derive(Serialize)]
struct TableRow {
    columns: Vec<TableCell>,
}

#[derive(Serialize)]
struct Table {
    headers: Vec<String>,
    rows: Vec<TableRow>,
}

enum TableCell {
    Month { year: i32, month: u32 },
    Value(Decimal),
    Text(String),
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
            TableCell::Text(val) => format!("{}", val),
        };
        serializer.serialize_str(&text)
    }
}

#[derive(Serialize)]
struct TreeNode {
    name: String,
    amount_main_commodity_value: Decimal,
    amount_main_commodity: String,
    amount_foreign_commodities: String,
    children: Vec<TreeNode>,
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

    let empty_balance = Balance::new();
    let total_balance = &monthly_report
        .monthly_balances
        .last()
        .map(|mb| &mb.total)
        .unwrap_or_else(|| &empty_balance);
    let summary_tree = get_summary_tree(total_balance, &input_data.prices, &report_params);
    data.insert("summary_tree".to_string(), to_json(&summary_tree));

    let table_months = get_table_months(&monthly_report, &input_data.prices, &report_params);
    data.insert("table_months".to_string(), to_json(&table_months));

    let mut area_chart1 = get_area_chart1(&table_months);
    data.insert("area_chart1".to_string(), to_json(&area_chart1));
    area_chart1.id = "areaChart2".to_string();
    data.insert("area_chart2".to_string(), to_json(&area_chart1));

    data
}

fn configure_html_header(data: &mut Map<String, Json>) {
    let mut style = include_str!("templates/charts/nv.d3.css").to_owned();
    style.push_str("\n");
    style.push_str(include_str!("templates/main.css"));

    let mut script = include_str!("templates/charts/d3.v3.js").to_owned();
    script.push_str("\n");
    script.push_str(include_str!("templates/charts/nv.d3.js"));
    script.push_str("\n");
    script.push_str(include_str!("templates/main.js"));

    data.insert("html_style".to_string(), to_json(style));
    data.insert("html_script".to_string(), to_json(script));
}

fn get_summary_tree(balance: &Balance, prices: &Prices, params: &ReportParameters) -> TreeNode {
    let src_tree_root= TreeBalanceNode::from(balance.clone());
    convert_tree_node("/", &src_tree_root, prices, params)
}

fn convert_tree_node(name: &str, src_node: &TreeBalanceNode,
                     prices: &Prices, params: &ReportParameters) -> TreeNode {
    let amount_main_commodity_value = src_node.balance.value_in_commodity_rounded(
        &params.main_commodity,
        params.main_commodity_decimal_points,
        Local::now().date().naive_local(),
        &prices,
    );

    let amount_main_commodity = format!("{} {}", amount_main_commodity_value, params.main_commodity);

    let amount_foreign_commodities = if src_node.balance.amounts.len() > 1 ||
        src_node.balance.amounts.len() == 1 && src_node.balance.amounts.iter().next().unwrap().0 != &params.main_commodity {
        src_node.balance.amounts.iter()
            .map(|a| format!("{}", a.1).to_string()).collect::<Vec<String>>().join(", ")
    } else {
        "".to_string()
    };

    let mut node = TreeNode {
        name: name.to_string(),
        amount_main_commodity_value,
        amount_main_commodity,
        amount_foreign_commodities,
        children: Vec::new(),
    };

    for (name, src_node) in &src_node.children {
        node.children.push(convert_tree_node(name, src_node, prices, params));
    }

    // remove empty (with 0 value) children
    node.children.retain(|n| n.amount_main_commodity_value != Decimal::zero());

    // sort children
    node.children.sort_by(|n1, n2| n1.name.cmp(&n2.name));

    node
}

fn get_table_months(
    monthly_report: &MonthlyReport,
    prices: &Prices,
    params: &ReportParameters,
) -> Table {
    let headers = vec![
        "Date".to_string(),
        "Assets Total Net".to_string(),
        "Liquid Assets".to_string(),
        "Fixed Assets".to_string(),
        "High Risk Assets Net".to_string(),
        "High Risk Assets Tax".to_string(),
        "Income".to_string(),
        "Expenses".to_string(),
    ];

    let mut rows: Vec<TableRow> = Vec::new();

    for monthly_balance in &monthly_report.monthly_balances {
        let last_day = last_day_in_month(monthly_balance.year, monthly_balance.month);

        let calc = MonthlyCalculator::new(&monthly_balance.total, &prices, last_day, &params);

        let tax = Decimal::new(32, 2);

        let assets_liquid = calc.get_value(&params.assets_liquid);
        let assets_fixed = calc.get_value(&params.assets_fixed);
        let assets_high_risk_net = (calc.get_value(&params.assets_high_risk)
            * (Decimal::new(1, 0) - tax))
            .round_dp_with_strategy(
                params.main_commodity_decimal_points,
                RoundingStrategy::RoundHalfUp,
            );
        let assets_high_risk_tax = (calc.get_value(&params.assets_high_risk) * tax)
            .round_dp_with_strategy(
                params.main_commodity_decimal_points,
                RoundingStrategy::RoundHalfUp,
            );
        let income = calc.get_value(&params.income);
        let expenses = calc.get_value(&params.expenses);

        rows.push(TableRow {
            columns: vec![
                TableCell::Month {
                    year: monthly_balance.year,
                    month: monthly_balance.month,
                },
                TableCell::Value(assets_liquid + assets_fixed + assets_high_risk_net),
                TableCell::Value(assets_liquid),
                TableCell::Value(assets_fixed),
                TableCell::Value(assets_high_risk_net),
                TableCell::Value(assets_high_risk_tax),
                TableCell::Value(income),
                TableCell::Value(expenses),
            ]}
        );
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
        self.balance
            .get_account_balance(&(accounts.as_deref()))
            .value_in_commodity_rounded(
                &self.params.main_commodity,
                self.params.main_commodity_decimal_points,
                self.last_day,
                &self.prices,
            )
    }
}

fn get_area_chart1(table_months: &Table) -> Chart {
    let min_date = table_months.rows[0].columns[0].to_timestamp_millis().unwrap();
    let max_date = table_months.rows.last().unwrap().columns[0]
        .to_timestamp_millis()
        .unwrap();

    let mut series_assets_liquid = Vec::new();
    let mut series_assets_fixed = Vec::new();
    let mut series_assets_high_risk_net = Vec::new();
    let mut series_assets_high_risk_tax = Vec::new();
    for row in &table_months.rows {
        let date = row.columns[0].to_timestamp_millis().unwrap() as f64;
        series_assets_liquid.push([date, row.columns[2].to_f64().unwrap()]);
        series_assets_fixed.push([date, row.columns[3].to_f64().unwrap()]);
        series_assets_high_risk_net.push([date, row.columns[4].to_f64().unwrap()]);
        series_assets_high_risk_tax.push([date, row.columns[5].to_f64().unwrap()]);
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
                key: "High Risk Assets Net".to_string(),
                values: series_assets_high_risk_net,
            },
            ChartSerie {
                key: "High Risk Assets Tax".to_string(),
                values: series_assets_high_risk_tax,
            },
        ])
        .to_string(),
    }
}
