use crate::report_data::structures::{Chart, ChartSerie};
use handlebars::to_json;
use crate::report_data::monthly_table::MonthlyTable;
use rust_decimal::prelude::ToPrimitive;

pub fn get_assets_chart(monthly_table: &MonthlyTable) -> Chart {
    let min_date = monthly_table.rows[0].date.and_hms(0, 0, 0)
        .timestamp_millis();
    let max_date = monthly_table.rows.last().unwrap().date.and_hms(0, 0, 0)
        .timestamp_millis();

    let mut series_assets_liquid = Vec::new();
    let mut series_assets_fixed = Vec::new();
    let mut series_assets_high_risk_net = Vec::new();
    let mut series_assets_high_risk_tax = Vec::new();
    for row in &monthly_table.rows {
        let date = row.date.and_hms(0, 0, 0)
            .timestamp_millis() as f64;
        series_assets_liquid.push([date, row.liquid_assets.to_f64().unwrap()]);
        series_assets_fixed.push([date, row.fixed_assets.to_f64().unwrap()]);
        series_assets_high_risk_net.push([date, row.high_risk_assets_net.to_f64().unwrap()]);
        series_assets_high_risk_tax.push([date, row.high_risk_assets_tax.to_f64().unwrap()]);
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
