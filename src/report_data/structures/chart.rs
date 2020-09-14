use serde::Serialize;

#[derive(Serialize)]
pub struct AreaChart {
    pub id: String,
    pub min_x: f64,
    pub max_x: f64,
    pub digit_points: u32,
    pub series: String,
}

#[derive(Serialize)]
pub struct AreaChartSerie {
    pub key: String,
    pub values: Vec<[f64; 2]>,
}

#[derive(Serialize)]
pub struct LineChart {
    pub id: String,
    pub min_x: f64,
    pub max_x: f64,
    pub digit_points: u32,
    pub series: String,
}

#[derive(Serialize)]
pub struct LineChartSerie {
    pub key: String,
    pub area: bool,
    pub values: Vec<[f64; 2]>,
}
