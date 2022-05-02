use std::{fmt::{Display, Error, Formatter}, time::{UNIX_EPOCH, Duration}};

use chrono::{DateTime, Utc};
use yahoo_finance_api::Quote;

static SEPARATOR: &str = ",";
#[derive(Debug)]
pub struct TickerSummary {
    pub last_date: u64,
    pub from_date: u64,
    pub price: f64,
    pub symbol: String,
    pub max: f64,
    pub min: f64,
    count: usize,
    pub start_price: f64,
    pub last_price: f64,
    running_total: f64,
}

impl TickerSummary {
    pub fn new(symbol: &str) -> Self {
        Self {
            last_date: 0,
            from_date: 0,
            price: 0.0,
            symbol: symbol.to_owned(),
            max: 0.0,
            min: 0.0,
            count: 0,
            start_price: 0.0,
            last_price: 0.0,
            running_total: 0.0,
        }
    }

    pub fn update_ticker_summary(&mut self, quotes: Vec<Quote>) {
        if !quotes.is_empty() {
            let mut first_quote = &quotes[0];
            let mut last_quote = &quotes[0];
            let mut running_total = 0.0;
            for quote in quotes.iter() {
                if quote.high > self.max {
                    self.max = quote.high;
                }
                if quote.low < self.min {
                    self.min = quote.low;
                }
                running_total += quote.adjclose;
                if quote.timestamp > last_quote.timestamp {
                    last_quote = &quote;
                }
                if quote.timestamp < first_quote.timestamp {
                    first_quote = &quote;
                }
            }
            self.running_total += running_total;
            self.count += quotes.len();
            self.last_date = last_quote.timestamp;
            self.from_date = first_quote.timestamp;
            self.price = last_quote.adjclose;
        }
    }

    pub fn avg(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        } else {
            let avg = self.running_total / self.count as f64;
            return Some(avg);
        }
    }

    pub fn diff(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        } else {
            return Some(self.last_price - self.start_price);
        }
    }

    pub fn diff_percent(&self) -> Option<f64> {
        if self.count == 0 || ((self.start_price - 0.0).abs() < f64::EPSILON) {
            return None;
        } else {
            let diff_percent = self.last_price * 100.0 / self.start_price;
            return Some(diff_percent);
        }
    }

    pub fn from_date(&self) -> DateTime<Utc> {
        DateTime::from(UNIX_EPOCH + Duration::from_secs(self.from_date))
    }

    pub fn last_date(&self) -> DateTime<Utc> {
        DateTime::from(UNIX_EPOCH + Duration::from_secs(self.last_date))
    }
}

impl Display for TickerSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.count == 0 {
            panic!(
                "Call to uninitialized TickerSummary with label {}",
                self.symbol
            );
        } else {
            let mut comma_separated = String::new();
            comma_separated.push_str(&format!("{}", &self.last_date().to_rfc3339()));
            comma_separated.push_str(SEPARATOR);
            comma_separated.push_str(&self.symbol);
            comma_separated.push_str(SEPARATOR);
            comma_separated.push_str(&format!("${:.2}", &self.price));
            comma_separated.push_str(SEPARATOR);
            comma_separated.push_str(&format!("{:.2}%", &self.diff_percent().unwrap()));
            comma_separated.push_str(SEPARATOR);
            comma_separated.push_str(&format!("${:.2}", &self.min));
            comma_separated.push_str(SEPARATOR);
            comma_separated.push_str(&format!("${:.2}", &self.max));
            comma_separated.push_str(SEPARATOR);
            comma_separated.push_str(&format!("${:.2}", &self.avg().unwrap()));
            write!(f, "{}", comma_separated)
        }
    }
}
