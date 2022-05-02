mod ticker_data;

mod prelude {
    pub use crate::ticker_data::*;
    pub use chrono::{prelude::*, Duration};
    pub use log::{error, warn};
    pub use std::{collections::HashMap, f64::INFINITY, process::exit};
    pub use yahoo_finance_api as yahoo;
}

use clap::{arg, command, Command};
use prelude::*;

fn is_valid_date(d: &str) -> Result<(), String> {
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

fn extract_date(date_str: &str) -> Result<DateTime<Utc>, String> {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => {
            match Utc.from_local_datetime(&date.and_hms(00, 00, 00)) {
                chrono::LocalResult::None => {
                    Err("Can not convert date to UTC datetime.".to_string())
                }
                chrono::LocalResult::Single(from_date) => Ok(from_date),
                chrono::LocalResult::Ambiguous(from_date, _) => Ok(from_date), //effectively earliest date
            }
        }
        Err(parse_error) => Err(format!("{:?}", parse_error)),
    }
}

fn parse_window_param<'a>(
    sma_matches: &clap::ArgMatches,
    from_date: &DateTime<Utc>,
) -> Result<usize, String> {
    // window parameter is mandatory. Therefore unwrap is safe here.
    let result = match sma_matches.value_of("window").unwrap().parse::<usize>() {
        Ok(value) => {
            let time_period = Utc::now() - *from_date;
            if time_period.num_days() >= value as i64 {
                return Ok(value);
            } else {
                return Err(format!(
                    "Time period {} is shorter than sliding window {}",
                    time_period, value
                ));
            }
        }
        Err(error) => Err(format!(
            "Can not parse parameter value of window to a number. {:?}",
            error
        )),
    };
    return result;
}

fn main() {
    env_logger::init();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(Command::new("max").about("get maximum closing price in a time period"))
        .subcommand(
            Command::new("min").about("get minimum adjusted closing price in a time period"),
        )
        .subcommand(
            Command::new("diff")
                .about("get percentage and absolute price difference for given time period"),
        )
        .subcommand(Command::new("sum").about("get 30 day summary of tickers"))
        .subcommand(
            Command::new("sma")
                .about("get sliding window average for n days in given time period")
                .arg(
                    arg!(-w --window <DAYS>)
                        .help("size of sliding window in days")
                        .required(true)
                        .validator(|d| d.parse::<usize>()),
                ),
        )
        .arg(
            arg!(-f --from <START_DATE>)
                .help(
                    "start date from which you want to collect the data. Defaults to last 30 days.",
                )
                .required(false)
                .validator(|d| is_valid_date(d)),
        )
        .arg(
            arg!(-t --ticker <SYMBOL>)
                .help("ticker symbol of the stock paper")
                .value_delimiter(',')
                .required(true),
        )
        .get_matches();

    // Tickers is a required parameter. Therefore it is safe to use unwrap
    let mut tickers = matches.values_of("ticker").unwrap();

    let from_date = match matches.value_of("from") {
        Some(date) => match extract_date(date) {
            Ok(from_date) => from_date,
            Err(reason) => {
                error!("Can not parse date for reason {:?}", reason);
                exit(-3);
            }
        },
        None => Utc::now() - Duration::days(30),
    };

    let provider = yahoo::YahooConnector::new();
    match matches.subcommand() {
        Some(("max", _)) => {
            let max_prices = get_max_prices(&mut tickers, &provider, &from_date);
            println!("Max prices:");
            max_prices.iter().for_each(|(key, value)| {
                println!("{}: {}", *key, value.max);
            });
        }
        Some(("min", _)) => {
            let min_prices = get_min_prices(&mut tickers, &provider, &from_date);
            println!("Min prices:");
            min_prices.iter().for_each(|(key, value)| {
                println!("{}: {}", *key, value.min);
            });
        }
        Some(("sma", sma_matches)) => {
            let window: usize = match parse_window_param(sma_matches, &from_date) {
                Ok(value) => value,
                Err(error) => {
                    error!(
                        "Can not parse sliding window parameter to number of days. Reason: {}",
                        error
                    );
                    exit(-2);
                }
            };
            println!("Sliding windows of {} days", window);
            match get_sma_windows(&mut tickers, &provider, &from_date, window) {
                Ok(smas) => smas.iter().for_each(|(key, values)| {
                    println!("{}: {:#?}", *key, values);
                }),
                Err(s) => {
                    error!("Sliding window failed because of {}", s);
                    exit(-3);
                }
            };
        }
        Some(("diff", _)) => {
            let price_differences = get_price_differences(&mut tickers, &provider, &from_date);
            println!("Ticker\tPercent\tDifference");
            price_differences.iter().for_each(|(key, summary)| {
                println!("{}:\t{:.2}%\t{:.2}", *key, summary.diff_percent().unwrap(), summary.diff().unwrap());
            });
        }
        Some(("sum", _)) => {
            let ticker_summary = get_ticker_summary(&mut tickers, &provider, &from_date);
            println!("period start,symbol,price,change %,min,max,30d avg");
            ticker_summary
                .iter()
                .for_each(|(_key, ticker)| println!("{}", ticker));
        }
        _ => unreachable!("Exhausted list of subcommands. Subcommand required prevents `None`"),
    };
}
