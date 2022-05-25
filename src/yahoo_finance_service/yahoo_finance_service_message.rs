use chrono::{DateTime, Utc};
use yahoo_finance_api::{Quote, YahooError};
use actix::prelude::*;

use super::granularity::Granularity;
pub struct YahooFinanceServiceMessage {
    pub ticker_symbol: String,
    pub from_date: DateTime<Utc>,
    pub granularity: Granularity,
}

impl Message for YahooFinanceServiceMessage {
    type Result = Result<Vec<Quote>, YahooError>;
}