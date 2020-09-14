use crate::report_data::monthly_table::MonthlyTable;
use crate::report_data::{LineChart, LineChartSerie};
use handlebars::to_json;
use rust_decimal::prelude::{ToPrimitive, Zero};
use rust_decimal::Decimal;

pub fn get_expenses_chart(monthly_table: &MonthlyTable) -> LineChart {
    let min_date = monthly_table.rows[0]
        .date
        .and_hms(0, 0, 0)
        .timestamp_millis();
    let max_date = monthly_table
        .rows
        .last()
        .unwrap()
        .date
        .and_hms(0, 0, 0)
        .timestamp_millis();

    let mut series_monthly_expenses = Vec::new();
    let mut series_monthly_expenses_sma = Vec::new();
    let sma_size = 12;
    for (pos, row) in monthly_table.rows.iter().enumerate() {
        let date = row.date.and_hms(0, 0, 0).timestamp_millis() as f64;
        let previous_expenses = if pos > 0 {
            monthly_table.rows[pos - 1].expenses
        } else {
            Decimal::zero()
        };
        series_monthly_expenses.push([date, (row.expenses - previous_expenses).to_f64().unwrap()]);

        let last_sma_expenses = if pos > sma_size {
            monthly_table.rows[pos - sma_size].expenses
        } else {
            Decimal::zero()
        };
        series_monthly_expenses_sma.push([
            date,
            (row.expenses - last_sma_expenses).to_f64().unwrap() / (sma_size as f64),
        ]);
    }

    LineChart {
        id: "expensesChart".to_string(),
        min_x: min_date as f64,
        max_x: max_date as f64,
        digit_points: 0,
        series: to_json(vec![
            LineChartSerie {
                key: "Monthly Expenses".to_string(),
                area: true,
                values: series_monthly_expenses.clone(),
            },
            LineChartSerie {
                key: "SMA12".to_string(),
                area: false,
                values: series_monthly_expenses_sma,
            },
        ])
        .to_string(),
    }
}
