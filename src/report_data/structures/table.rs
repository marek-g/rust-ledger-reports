use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Serialize)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
}

#[derive(Serialize)]
pub struct TableRow {
    pub columns: Vec<TableCell>,
}

pub enum TableCell {
    Month { year: i32, month: u32 },
    Value(Decimal),
    Text(String),
}

impl serde::Serialize for TableCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        let text = match self {
            TableCell::Month { year, month } => format!("{}/{:02}", year, month),
            TableCell::Value(val) => format!("{}", val),
            TableCell::Text(val) => format!("{}", val),
        };
        serializer.serialize_str(&text)
    }
}
