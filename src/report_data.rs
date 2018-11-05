use handlebars::to_json;
use input_data::*;
use ledger_utils::monthly_report::*;
use serde_json::json;
use serde_json::value::{Map, Value as Json};

#[derive(Serialize)]
struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

pub fn make_report_data(input_data: &InputData) -> Map<String, Json> {
    let mut data = Map::new();

    //let balances = get_balance(&input_data.ledger);
    //println!("{:?}", balances);

    let monthly_report = MonthlyReport::from(&input_data.ledger);

    let table_months = get_table_months(&monthly_report);
    data.insert("table_months".to_string(), to_json(&table_months));

    data.insert("year".to_string(), to_json(format!("{:?}", monthly_report)));

    //println!("{:?}", data);

    data
}

fn get_table_months(monthly_report: &MonthlyReport) -> Table {
    let headers = vec!["Date".to_string(), "Cash".to_string()];

    let mut rows: Vec<Vec<String>> = Vec::new();

    for monthly_balance in &monthly_report.monthly_balances {
        let date = format!("{}/{:02}", monthly_balance.year, monthly_balance.month);
        let cash = if monthly_balance
            .total
            .account_balances
            .contains_key(&"Aktywa:Płynne:Gotówka".to_string())
        {
            format!(
                "{} PLN",
                monthly_balance.total.account_balances[&"Aktywa:Płynne:Gotówka".to_string()]
                    .amounts[&"PLN".to_string()]
                    .quantity
            )
        } else {
            "-".to_string()
        };

        rows.push(vec![date, cash]);
    }

    Table {
        headers: headers,
        rows: rows,
    }
}
