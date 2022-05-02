use std::fmt::{Display, Error, Formatter};

use chrono::{DateTime, Utc};

static SEPARATOR: &str = ",";
#[derive(Debug)]
pub struct TickerSummary {
    pub last_date: DateTime<Utc>,
    pub from_date: DateTime<Utc>,
    pub price: f64,
    pub symbol: String,
    pub max: f64,
    pub min: f64,
    pub perc: f64,
    pub diff: f64,
    pub avg: f64,
}

impl TickerSummary {
    pub fn new(symbol: &str, from_date: DateTime<Utc>) -> Self {
        Self {
            last_date: Utc::now(),
            from_date: from_date,
            price: 0.0,
            symbol: symbol.to_owned(),
            max: 0.0,
            min: 0.0,
            perc: 0.0,
            diff: 0.0,
            avg: 0.0,
        }
    }
}

impl Display for TickerSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut comma_separated = String::new();
        comma_separated.push_str(&format!("{}", &self.last_date.to_rfc3339()));
        comma_separated.push_str(SEPARATOR);
        comma_separated.push_str(&self.symbol);
        comma_separated.push_str(SEPARATOR);
        comma_separated.push_str(&format!("${:.2}", &self.price));
        comma_separated.push_str(SEPARATOR);
        comma_separated.push_str(&format!("{:.2}%",&self.perc));
        comma_separated.push_str(SEPARATOR);
        comma_separated.push_str(&format!("${:.2}",&self.min));
        comma_separated.push_str(SEPARATOR);
        comma_separated.push_str(&format!("${:.2}", &self.max));
        comma_separated.push_str(SEPARATOR);
        comma_separated.push_str(&format!("${:.2}",&self.avg));
        write!(f, "{}", comma_separated)
    }
}
