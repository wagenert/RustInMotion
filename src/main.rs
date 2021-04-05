mod ticker_data;

mod prelude {
    pub use crate::ticker_data::*;
    pub use chrono::{prelude::*, Duration};
    pub use clap::{App, Arg, SubCommand};
    pub use std::{collections::HashMap, f64::INFINITY, process::exit};
    pub use yahoo_finance_api as yahoo;
}

use prelude::*;

fn is_valid_date(d: String) -> Result<(), String> {
    match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
        Ok(date) => {
            if Utc.from_utc_date(&date) > Utc::today() {
                Err(format!("Start date is in the future."))
            } else {
                Ok(())
            }
        }
        Err(error) => Err(format!(
            "Can not parse <START DATE> to a valid date. {}",
            error
        )),
    }
}

fn extract_date(date_str: &str) -> DateTime<Utc> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let from_date = Utc
        .from_local_datetime(&date.and_hms(00, 00, 00))
        .earliest()
        .unwrap();
    from_date
}

fn parse_window_param<'a>(sma_matches: Option<&clap::ArgMatches>) -> Result<usize, String> {
    if let Some(matches) = sma_matches {
        let result = match matches.value_of("window").unwrap().parse::<usize>() {
            Ok(value) => Ok(value),
            Err(error) => Err(format!(
                "Can not parse parameter value of window to a number. {:?}",
                error
            )),
        };
        return result;
    }
    Err(String::from("Missing parameter window!"))
}

fn main() {
    let matches = App::new("Stock data processor")
        .version("0.1")
        .author("Thomas Wagener <thomas.wagener@web.de")
        .about("Read data for a stock values from a given start date in days")
        .subcommand(
            SubCommand::with_name("max")
                .about("get maximum closing price in a time period")
                .version("0.1"),
        )
        .subcommand(
            SubCommand::with_name("min")
                .about("get minimum adjusted closing price in a time period")
                .version("0.1"),
        )
        .subcommand(
            SubCommand::with_name("sma")
                .about("get sliding window average for n days in given time period")
                .version("0.1")
                .arg(
                    Arg::with_name("window")
                        .short("w")
                        .long("window")
                        .value_name("DAYS")
                        .help("size of sliding window in days")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("diff")
                .about("get percentage and absolut price difference for given time period")
                .version("0.1"),
        )
        .subcommand(
            SubCommand::with_name("sum")
                .about("get 30 day summary of tickers")
                .version("0.1"),
        )
        .arg(
            Arg::with_name("ticker")
                .short("t")
                .long("ticker")
                .value_name("SYMBOL")
                .help("ticker symbol of the stock paper")
                .value_delimiter(",")
                .required(true),
        )
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("START DATE")
                .help(
                    "start date from which you want to collect the data. Defaults to last 30 days.",
                )
                .validator(is_valid_date),
        )
        .get_matches();

    let mut tickers = matches.values_of("ticker").unwrap();
    let from_date = match matches.value_of("from") {
        Some(date) => extract_date(date),
        None => Utc::now() - Duration::days(30),
    };

    let provider = yahoo::YahooConnector::new();
    match matches.subcommand() {
        ("max", _) => {
            let max_prices = get_max_prices(&mut tickers, &provider, &from_date);
            println!("Max prices:");
            max_prices.iter().for_each(|(key, value)| {
                println!("{}: {}", *key, value);
            });
        }
        ("min", _) => {
            let min_prices = get_min_prices(&mut tickers, &provider, &from_date);
            println!("Min prices:");
            min_prices.iter().for_each(|(key, value)| {
                println!("{}: {}", *key, value);
            });
        }
        ("sma", sma_matches) => {
            let window: usize = match parse_window_param(sma_matches) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{}", error);
                    exit(-2);
                }
            };
            let smas: HashMap<&str, Vec<f64>> =
                get_sma_windows(&mut tickers, &provider, &from_date, window);
            println!("Sliding windows of {} days", window);
            smas.iter().for_each(|(key, values)| {
                println!("{}: {:#?}", *key, values);
            });
        }
        ("diff", _) => {
            let price_differences: HashMap<&str, (f64, f64)> =
                get_price_differences(&mut tickers, &provider, &from_date);
            println!("Ticker\tPercent\tDifference");
            price_differences.iter().for_each(|(key, (perc, diff))| {
                println!("{}:\t{:.2}%\t{:.2}", *key, perc, diff);
            });
        }
        ("sum", _) => {
            let ticker_summary = get_ticker_summary(&mut tickers, &provider, &from_date);
            println!("period start,symbol,price,change %,min,max,30d avg");
            ticker_summary.iter().for_each(|(_key, ticker)| println!("{}", ticker));
        }
        _ => {
            eprintln!("Unknown command!");
            exit(-1);
        }
    };
}
