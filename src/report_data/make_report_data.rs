use crate::configuration::{ReportParameters};
use handlebars::to_json;
use crate::ledger_utils::balance::Balance;
use crate::ledger_utils::monthly_report::*;
use serde_json::value::{Map, Value as Json};
use crate::report_data::*;
use ledger_parser::Ledger;
use crate::ledger_utils::prices::Prices;

pub fn make_report_data(
    ledger: &Ledger,
    prices: &Prices,
    report_params: &ReportParameters,
) -> Map<String, Json> {
    let mut data = Map::new();

    configure_html_header(&mut data);

    let monthly_report = MonthlyReport::from(ledger);

    let empty_balance = Balance::new();
    let total_balance = &monthly_report
        .monthly_balances
        .last()
        .map(|mb| &mb.total)
        .unwrap_or_else(|| &empty_balance);
    let summary_tree = get_summary_tree(total_balance, &prices, &report_params);
    data.insert("summary_tree".to_string(), to_json(&summary_tree));

    let monthly_table = get_monthly_table(&monthly_report, &prices, &report_params);

    let assets_table = get_assets_table(&monthly_table);
    data.insert("assets_table".to_string(), to_json(&assets_table));

    let assets_chart = get_assets_chart(&monthly_table);
    data.insert("assets_chart".to_string(), to_json(&assets_chart));

    let expenses_chart = get_expenses_chart(&monthly_table);
    data.insert("expenses_chart".to_string(), to_json(&expenses_chart));

    data
}

fn configure_html_header(data: &mut Map<String, Json>) {
    let mut style = include_str!("../templates/charts/nv.d3.css").to_owned();
    style.push_str("\n");
    style.push_str(include_str!("../templates/main.css"));

    let mut script = include_str!("../templates/charts/d3.v3.js").to_owned();
    script.push_str("\n");
    script.push_str(include_str!("../templates/charts/nv.d3.js"));
    script.push_str("\n");
    script.push_str(include_str!("../templates/main.js"));

    data.insert("html_style".to_string(), to_json(style));
    data.insert("html_script".to_string(), to_json(script));
}
