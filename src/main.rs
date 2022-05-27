extern crate clap;

use chrono;
use clap::{Arg, Command};
use reqwest;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::process::Command as procCommand;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let matches = Command::new("ledgerdb")
		.author("Adithya Chari, <adithya.chari@gmail.com>")
		.version("1.0.0")
		.about("Gets price updates for stock ticker symbols in your ledger file")
		.arg(
			Arg::new("file")
				.short('f')
				.long("file")
				.required(true)
				.takes_value(true)
				.value_name("LEDGER_FILE")
				.help("Takes the filepath of your ledger-cli file")
		)
		.arg(
			Arg::new("database")
				.short('p')
				.long("pricedb")
				.required(true)
				.takes_value(true)
				.value_name("PRICE_DATABASE")
				.help("Takes the filepath of your ledger-cli price database (or the path to create it at)")
		)
		.arg(
			Arg::new("token")
				.short('a')
				.long("api-token")
				.required(true)
				.takes_value(true)
				.value_name("API_TOKEN")
				.help("Takes your AlphaVantage API token, available for free at the website")
		).get_matches();

    let file = matches.value_of("file").unwrap();
    let pricedb = matches.value_of("database").unwrap();
    let token = matches.value_of("token").unwrap();

    let list = get_commodities(file);
    let map = get_all_prices(list, token);

    let mut db_file = File::options()
        .append(true)
        .create(true)
        .open(pricedb)
        .unwrap();

    for (ticker, price) in map.iter() {
        let timestamp = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");

        db_file
            .write_all(format!("P {} {} ${:.2}\n", timestamp, ticker, price).as_bytes())
            .expect("Failed to write");
    }
}

fn get_all_prices(commodities: Vec<String>, token: &str) -> BTreeMap<String, f64> {
    let mut map: BTreeMap<String, f64> = BTreeMap::new();

    let mut start_time = Instant::now();
    for (i, commodity) in commodities.iter().enumerate() {
        if (i + 1) % 5 == 0 {
            let elapsed_time = Instant::now().duration_since(start_time);
            if elapsed_time < Duration::from_secs(60) {
                thread::sleep(Duration::from_secs(60) - elapsed_time);
            }

            start_time = Instant::now();
        }

        map.insert(commodity.to_string(), get_price(commodity, token));
    }

    map
}

fn get_price(commodity: &str, token: &str) -> f64 {
    let res = reqwest::blocking::get(format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        commodity, token
    ))
    .unwrap();
    let body = res.text().unwrap();

    let json: Value = serde_json::from_str(&body).unwrap();
    json.pointer("/Global Quote/05. price")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap()
}

fn get_commodities(file: &str) -> Vec<String> {
    let output = procCommand::new("ledger")
        .arg("-f")
        .arg(file)
        .arg("commodities")
        .output()
        .expect("Failed to call ledger");

    let list_str: String = String::from_utf8(output.stdout).unwrap();
    list_str
        .lines()
        .filter(|&s| is_ticker(s))
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn is_ticker(commodity: &str) -> bool {
    commodity
        .chars()
        .all(|c| c == '.' || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9'))
}
