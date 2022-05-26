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
    ledgerdb -f [ledger file] -p [price database file (to create or update)] -a [AlphaVantage API Token] -b [Name of ledger binary]
```
