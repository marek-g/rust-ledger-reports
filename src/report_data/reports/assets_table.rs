use crate::report_data::structures::{Table, TableRow, TableCell};
use crate::report_data::monthly_table::MonthlyTable;
use chrono::Datelike;

pub fn get_assets_table(
    monthly_table: &MonthlyTable,
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

    for row in &monthly_table.rows {
        rows.push(TableRow {
            columns: vec![
                TableCell::Month {
                    year: row.date.year(),
                    month: row.date.month(),
                },
                TableCell::Value(row.assets_total_net),
                TableCell::Value(row.liquid_assets),
                TableCell::Value(row.fixed_assets),
                TableCell::Value(row.high_risk_assets_net),
                TableCell::Value(row.high_risk_assets_tax),
                TableCell::Value(row.income),
                TableCell::Value(row.expenses),
            ]}
        );
    }

    Table {
        headers,
        rows,
    }
}
