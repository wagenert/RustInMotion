mod data_processing;
mod granularity;
mod ticker_summary;

use crate::prelude::*;
use data_processing::*;
use granularity::*;
use ticker_summary::*;
use yahoo::{Quote, YResponse, YahooConnector, YahooError};

fn get_quotes(
    provider: &YahooConnector,
    ticker: &str,
    from_date: &DateTime<Utc>,
    granularity: &Granularity,
) -> Result<Vec<Quote>, YahooError> {
    match provider.get_quote_history_interval(
        ticker,
        from_date.clone(),
        Utc::now(),
        granularity.to_string(),
    ) {
        Ok(response) => response.quotes(),
        Err(error) => Err(error),
    }
}

fn get_prices_from_response(response: YResponse) -> Result<Vec<f64>, YahooError> {
    match response.quotes() {
        Ok(quotes) => Ok(quotes.iter().map(|quote| quote.adjclose).collect()),
        Err(yerr) => Err(yerr),
    }
}

fn get_prices(
    provider: &YahooConnector,
    ticker: &str,
    from_date: &DateTime<Utc>,
    granularity: &Granularity,
) -> Result<Vec<f64>, YahooError> {
    match get_quotes(provider, ticker, from_date, granularity) {
        Ok(quotes) => Ok(quotes.iter().map(|quote| quote.adjclose).collect()),
        Err(yerr) => Err(yerr),
    }
}

pub fn get_max_prices<'a>(
    tickers: &'a mut clap::Values,
    provider: &'a yahoo::YahooConnector,
    from_date: &'a DateTime<Utc>,
) -> HashMap<&'a str, TickerSummary> {
    let granularity = Granularity::Day;
    let mut result = HashMap::<&str, TickerSummary>::new();
    for ticker in tickers {
        match provider.get_quote_history_interval(
            ticker,
            from_date.clone(),
            Utc::now(),
            granularity.to_string(),
        ) {
            Ok(response) => match response.quotes() {
                Ok(quotes) => {
                    let mut ticker_data = TickerSummary::new(ticker);
                    ticker_data.update_ticker_summary(quotes);
                    result.insert(ticker, ticker_data);
                },
                Err(_) => todo!(),
            },
            Err(error) => {
                warn!(
                    "Cannot retrieve response for ticker {}! {:?}",
                    ticker, error
                );
                continue;
            }
        };
        /* let quotes = match get_prices(provider, ticker, from_date, &granularity) {
            Ok(value) => value,
            Err(_) => {
                warn!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        let max_price = max(quotes.as_slice()).unwrap();
        result.insert(ticker, max_price); */
    }
    result
}

pub fn get_min_prices<'a>(
    tickers: &'a mut clap::Values,
    provider: &'a yahoo::YahooConnector,
    from_date: &'a DateTime<Utc>,
) -> HashMap<&'a str, TickerSummary> {
    let granularity = Granularity::Day;
    let mut result = HashMap::<&str, TickerSummary>::new();
    for ticker in tickers {
        match provider.get_quote_history_interval(
            ticker,
            from_date.clone(),
            Utc::now(),
            granularity.to_string(),
        ) {
            Ok(response) => match response.quotes() {
                Ok(quotes) => {
                    let mut ticker_data = TickerSummary::new(ticker);
                    ticker_data.update_ticker_summary(quotes);
                    result.insert(ticker, ticker_data);
                },
                Err(_) => todo!(),
            },
            Err(error) => {
                warn!(
                    "Cannot retrieve response for ticker {}! {:?}",
                    ticker, error
                );
                continue;
            }
        };
        /* let quotes = match get_prices(provider, ticker, from_date, &granularity) {
            Ok(value) => value,
            Err(_) => {
                warn!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        let min_price = min(quotes.as_slice()).unwrap();
        result.insert(ticker, min_price); */
    }
    result
}

pub fn get_sma_windows<'a>(
    tickers: &'a mut clap::Values,
    provider: &'a yahoo::YahooConnector,
    from_date: &DateTime<Utc>,
    window: usize,
) -> Result<HashMap<&'a str, Vec<f64>>, String> {
    let granularity = Granularity::Day;
    let mut result = HashMap::new();
    for ticker in tickers {
        let quotes = match get_prices(provider, ticker, from_date, &granularity) {
            Ok(value) => value,
            Err(_) => {
                warn!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        if let Some(sla) = n_window_sma(window, quotes.as_slice()) {
            result.insert(ticker, sla);
        } else {
            return Err("Slinding window did return None as a result.".to_string());
        }
    }
    Ok(result)
}

pub fn get_price_differences<'a>(
    tickers: &'a mut clap::Values,
    provider: &yahoo::YahooConnector,
    from_date: &DateTime<Utc>,
) -> HashMap<&'a str, TickerSummary> {
    let granularity = Granularity::Day;
    let mut result = HashMap::new();
    for ticker in tickers {
        match provider.get_quote_history_interval(
            ticker,
            from_date.clone(),
            Utc::now(),
            granularity.to_string(),
        ) {
            Ok(response) => match response.quotes() {
                Ok(quotes) => {
                    let mut ticker_data = TickerSummary::new(ticker);
                    ticker_data.update_ticker_summary(quotes);
                    result.insert(ticker, ticker_data);
                },
                Err(_) => todo!(),
            },
            Err(error) => {
                warn!(
                    "Cannot retrieve response for ticker {}! {:?}",
                    ticker, error
                );
                continue;
            }
        };
/*         let quotes = match get_prices(provider, ticker, from_date, &granularity) {
            Ok(value) => value,
            Err(_) => {
                warn!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        if let Some(difference) = price_difference(quotes.as_slice()) {
            result.insert(ticker, difference);
        } else {
            warn!("Could not calculate difference for {}. Skipping!", ticker);
        }
 */    }
    result
}

pub fn get_ticker_summary<'a>(
    tickers: &'a mut clap::Values,
    provider: &YahooConnector,
    from_date: &DateTime<Utc>,
) -> HashMap<&'a str, TickerSummary> {
    let mut result = HashMap::new();
    let granularity = Granularity::Day;
    for ticker in tickers {
        match provider.get_quote_history_interval(
            ticker,
            from_date.clone(),
            Utc::now(),
            granularity.to_string(),
        ) {
            Ok(response) => match response.quotes() {
                Ok(quotes) => {
                    let mut ticker_data = TickerSummary::new(ticker);
                    ticker_data.update_ticker_summary(quotes);
                    result.insert(ticker, ticker_data);
                },
                Err(_) => todo!(),
            },
            Err(error) => {
                warn!(
                    "Cannot retrieve response for ticker {}! {:?}",
                    ticker, error
                );
                continue;
            }
        };
    }
    result
}
