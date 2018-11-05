use chrono::*;

/// Returns last day in a particular month.
pub fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd(year, month, ndays_in_month(year, month))
}

/// Determine the number of days in a particular month.
pub fn ndays_in_month(year: i32, month: u32) -> u32 {
    // the first day of the next month...
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let d = NaiveDate::from_ymd(y, m, 1);

    // ...is preceded by the last day of the original month
    d.pred().day()
}
