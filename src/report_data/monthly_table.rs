use rust_decimal::{Decimal, RoundingStrategy};
use crate::ledger_utils::monthly_report::MonthlyReport;
use crate::ledger_utils::prices::Prices;
use crate::configuration::{ReportParameters, VecDeref};
use crate::date_utils::last_day_in_month;
use crate::ledger_utils::balance::Balance;
use chrono::NaiveDate;

pub struct MonthlyTable {
    pub rows: Vec<MonthlyRow>,
}

pub struct MonthlyRow {
    pub date: NaiveDate,
    pub assets_total_net: Decimal,
    pub liquid_assets: Decimal,
    pub fixed_assets: Decimal,
    pub high_risk_assets_net: Decimal,
    pub high_risk_assets_tax: Decimal,
    pub income: Decimal,
    pub expenses: Decimal,
}

pub fn get_monthly_table(
    monthly_report: &MonthlyReport,
    prices: &Prices,
    params: &ReportParameters,
) -> MonthlyTable {
    let mut rows: Vec<MonthlyRow> = Vec::new();

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

        rows.push(MonthlyRow {
            date: last_day,
            assets_total_net: assets_liquid + assets_fixed + assets_high_risk_net,
            liquid_assets: assets_liquid,
            fixed_assets: assets_fixed,
            high_risk_assets_net: assets_high_risk_net,
            high_risk_assets_tax: assets_high_risk_tax,
            income,
            expenses,
        });
    }

    MonthlyTable {
        rows,
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
            balance,
            prices,
            last_day,
            params,
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
