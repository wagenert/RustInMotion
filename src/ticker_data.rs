mod data_processing;
mod granularity;
mod ticker_summary;

use crate::prelude::*;

use data_processing::*;
use granularity::*;

use ticker_summary::*;
use yahoo::{Quote, YahooConnector, YahooError};

async fn get_quotes(
    ticker: String,
    from_date: DateTime<Utc>,
    granularity: &'static Granularity,
) -> Result<Vec<Quote>, YahooError> {
    let provider = yahoo::YahooConnector::new();
    match provider
        .get_quote_history_interval(
            &ticker,
            from_date.clone(),
            Utc::now(),
            granularity.to_string(),
        )
        .await
    {
        Ok(response) => response.quotes(),
        Err(error) => Err(error),
    }
}

async fn get_prices(
    ticker: String,
    from_date: DateTime<Utc>,
    granularity: &'static Granularity,
) -> Result<Vec<f64>, YahooError> {
    match actix_rt::spawn(get_quotes(ticker, from_date, granularity))
        .await
        .unwrap()
    {
        Ok(quotes) => Ok(quotes.iter().map(|quote| quote.adjclose).collect()),
        Err(yerr) => Err(yerr),
    }
}

pub async fn get_max_prices(
    tickers: Vec<String>,
    from_date: DateTime<Utc>,
) -> HashMap<String, f64> {
    let mut result = HashMap::<String, f64>::new();
    for ticker in tickers {
        let quotes = match get_prices(ticker.clone(), from_date, &Granularity::Day)
            .await
        {
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
    tickers: Vec<String>,
    from_date: DateTime<Utc>,
) -> HashMap<String, f64> {
    let mut result = HashMap::<String, f64>::new();
    for ticker in tickers {
        let quotes = match actix_rt::spawn(get_prices(ticker.clone(), from_date, &Granularity::Day))
            .await
            .unwrap()
        {
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
    tickers: Vec<String>,
    from_date: DateTime<Utc>,
    window: usize,
) -> Result<HashMap<String, Vec<f64>>, String> {
    let mut result = HashMap::new();
    for ticker in tickers {
        let quotes = match actix_rt::spawn(get_prices(ticker.clone(), from_date, &Granularity::Day))
            .await
            .unwrap()
        {
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
    tickers: Vec<String>,
    from_date: DateTime<Utc>,
) -> HashMap<String, (f64, f64)> {
    let mut result = HashMap::new();
    for ticker in tickers {
        let quotes = match get_prices(ticker.clone(), from_date, &Granularity::Day)
            .await
        {
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
    tickers: Vec<String>,
    from_date: DateTime<Utc>,
) -> HashMap<String, TickerSummary> {
    let mut result = HashMap::new();
    let granularity: Granularity = Granularity::Day;
    let provider = YahooConnector::new();
    for ticker in tickers {
        match provider.get_quote_history_interval(
                &ticker,
                from_date.clone(),
                Utc::now(),
                granularity.to_string(),
            )
            .await
        {
            Ok(response) => match response.quotes() {
                Ok(quotes) => {
                    let mut ticker_data = TickerSummary::new(&ticker);
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
