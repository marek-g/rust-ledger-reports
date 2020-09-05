use serde::Serialize;

#[derive(Serialize)]
pub struct Chart {
    pub id: String,
    pub min_x: f64,
    pub max_x: f64,
    pub digit_points: u32,
    pub series: String,
}

#[derive(Serialize)]
pub struct ChartSerie {
    pub key: String,
    pub values: Vec<[f64; 2]>,
}
