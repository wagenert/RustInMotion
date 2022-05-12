mod data_processing;
mod granularity;
mod ticker_summary;

use crate::prelude::*;
use clap::Values;
use data_processing::*;
use granularity::*;

use ticker_summary::*;
use yahoo::{Quote, YahooConnector, YahooError};

async fn get_quotes(
    provider: &'static YahooConnector,
    ticker: &'static str,
    from_date: &DateTime<Utc>,
    granularity: &'static Granularity,
) -> Result<Vec<Quote>, YahooError> {
    match tokio::spawn(provider.get_quote_history_interval(
        ticker,
        from_date.clone(),
        Utc::now(),
        granularity.to_string(),
    ))
    .await
    .unwrap()
    {
        Ok(response) => response.quotes(),
        Err(error) => Err(error),
    }
}

async fn get_prices(
    provider: &'static YahooConnector,
    ticker: &'static str,
    from_date: &'static DateTime<Utc>,
    granularity: &'static Granularity,
) -> Result<Vec<f64>, YahooError> {
    match tokio::spawn(get_quotes(provider, ticker, from_date, granularity))
        .await
        .unwrap()
    {
        Ok(quotes) => Ok(quotes.iter().map(|quote| quote.adjclose).collect()),
        Err(yerr) => Err(yerr),
    }
}

pub async fn get_max_prices(
    tickers: &mut Values<'static>,
    provider: &'static yahoo::YahooConnector,
    from_date: &'static DateTime<Utc>,
) -> HashMap<&'static str, f64> {
    static granularity: Granularity = Granularity::Day;
    let mut result = HashMap::<&str, f64>::new();
    for ticker in tickers {
        let quotes = match tokio::spawn(get_prices(provider, ticker, from_date, &granularity)).await.unwrap() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        let max_price = max(quotes.as_slice()).unwrap();
        result.insert(ticker, max_price);
    }
    result
}

pub async fn get_min_prices(
    tickers: &mut Values<'static>,
    provider: &'static yahoo::YahooConnector,
    from_date: &'static DateTime<Utc>,
) -> HashMap<&'static str, f64> {
    static granularity: Granularity = Granularity::Day;
    let mut result = HashMap::<&str, f64>::new();
    for ticker in tickers {
        let quotes = match tokio::spawn(get_prices(provider, ticker, from_date, &granularity)).await.unwrap() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        let min_price = min(quotes.as_slice()).unwrap();
        result.insert(ticker, min_price);
    }
    result
}

pub async fn get_sma_windows(
    tickers: &mut Values<'static>,
    provider: &'static yahoo::YahooConnector,
    from_date: &'static DateTime<Utc>,
    window: usize,
) -> Result<HashMap<&'static str, Vec<f64>>, String> {
    static granularity: Granularity = Granularity::Day;
    let mut result = HashMap::new();
    for ticker in tickers {
        let quotes = match tokio::spawn(get_prices(provider, ticker, from_date, &granularity)).await.unwrap() {
            Ok(value) => value,
            Err(_) => {
                warn!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        if let Some(sla) = n_window_sma(window, quotes.as_slice()) {
            result.insert(ticker, sla);
        } else {
            return Err("Sliding window did return None as a result.".to_string());
        }
    }
    Ok(result)
}

pub async fn get_price_differences(
    tickers: &mut Values<'static>,
    provider: &'static yahoo::YahooConnector,
    from_date: &'static DateTime<Utc>,
) -> HashMap<&'static str, (f64, f64)> {
    static granularity: Granularity = Granularity::Day;
    let mut result = HashMap::new();
    for ticker in tickers {
        let quotes = match tokio::spawn(get_prices(provider, ticker, from_date, &granularity)).await.unwrap() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to retrieve quotes for ticker {}", ticker);
                continue;
            }
        };
        if let Some(difference) = price_difference(quotes.as_slice()) {
            result.insert(ticker, difference);
        } else {
            eprintln!("Could not calculate difference for {}. Skipping!", ticker);
        }
    }
    result
}

pub async fn get_ticker_summary(
    tickers: &mut Values<'static>,
    provider: &'static YahooConnector,
    from_date: &DateTime<Utc>,
) -> HashMap<&'static str, TickerSummary> {
    let mut result = HashMap::new();
    static granularity: Granularity = Granularity::Day;
    for ticker in tickers {
        match tokio::spawn(provider.get_quote_history_interval(
            ticker,
            from_date.clone(),
            Utc::now(),
            granularity.to_string(),
        )).await.unwrap() {
            Ok(response) => match response.quotes() {
                Ok(quotes) => {
                    let mut ticker_data = TickerSummary::new(ticker);
                    ticker_data.update_ticker_summary(quotes);
                    result.insert(ticker, ticker_data);
                }
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
