use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Serialize)]
pub struct TreeNode {
    pub name: String,
    pub is_positive: bool,
    pub amount_main_commodity_value: Decimal,
    pub amount_main_commodity: String,
    pub amount_foreign_commodities: String,
    pub children: Vec<TreeNode>,
}
