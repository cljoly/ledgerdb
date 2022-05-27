# LedgerDB

This application locates any stocks you have in your [ledger-cli](https://ledger-cli.org) file, then generates a price database of those stocks compatible with the application.

### Installation

This package has been published to crates.io, and is installable with cargo.

```
	cargo install ledgerdb
```

### Usage

```
USAGE:
    ledgerdb --file <LEDGER_FILE> --pricedb <PRICE_DATABASE> --api-token <API_TOKEN>

OPTIONS:
    -a, --api-token <API_TOKEN>       Takes your AlphaVantage API token, available for free at the
                                      website
    -f, --file <LEDGER_FILE>          Takes the filepath of your ledger-cli file
    -h, --help                        Print help information
    -p, --pricedb <PRICE_DATABASE>    Takes the filepath of your ledger-cli price database (or the
                                      path to create it at)
    -V, --version                     Print version information
```
