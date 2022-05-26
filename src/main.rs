extern crate clap;

use clap::{App, Arg};

fn main() -> Result<(), String> {
    let matches = App::new("ledgerdb")
		.author("Adithya Chari, <adithya.chari@gmail.com>")
		.version("1.0.0")
		.about("Gets price updates for stock ticker symbols in your ledger file")
		.arg(
			Arg::with_name("file")
				.short("f")
				.long("file")
				.required(true)
				.takes_value(true)
				.value_name("LEDGER_FILE")
				.help("Takes the filepath of your ledger-cli file")
		)
		.arg(
			Arg::with_name("database")
				.short("p")
				.long("pricedb")
				.required(true)
				.takes_value(true)
				.value_name("PRICE_DATABASE")
				.help("Takes the filepath of your ledger-cli price database (or the path to create it at)")
		)
		.arg(
			Arg::with_name("token")
				.short("a")
				.long("api-token")
				.required(true)
				.takes_value(true)
				.value_name("API_TOKEN")
				.help("Takes your AlphaVantage API token")
		).get_matches();
}

// fn getCommodities()
