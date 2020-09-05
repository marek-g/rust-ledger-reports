use crate::ledger_utils::balance::Balance;
use crate::ledger_utils::prices::Prices;
use crate::configuration::ReportParameters;
use crate::report_data::structures::TreeNode;
use crate::ledger_utils::tree_balance::TreeBalanceNode;
use chrono::Local;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

pub fn get_summary_tree(balance: &Balance, prices: &Prices, params: &ReportParameters) -> TreeNode {
    let src_tree_root= TreeBalanceNode::from(balance.clone());
    convert_tree_node("/", &src_tree_root, prices, params)
}

fn convert_tree_node(name: &str, src_node: &TreeBalanceNode,
                     prices: &Prices, params: &ReportParameters) -> TreeNode {
    let mut name = name.to_string();

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

    let mut children= Vec::new();

    for (name, src_node) in &src_node.children {
        children.push(convert_tree_node(name, src_node, prices, params));
    }

    // remove empty (with 0 value) children
    children.retain(|n| n.amount_main_commodity_value != Decimal::zero());

    // sort children
    children.sort_by(|n1, n2| n1.name.cmp(&n2.name));

    // if there is only one child, merge it
    if children.len() == 1 {
        if let Some(child) = children.iter().next() {
            if amount_main_commodity_value == child.amount_main_commodity_value {
                name = format!("{}:{}", name, child.name);
                children.clear();
            }
        }
    }

    TreeNode {
        name,
        is_positive: amount_main_commodity_value > Decimal::zero(),
        amount_main_commodity_value,
        amount_main_commodity,
        amount_foreign_commodities,
        children,
    }
}
