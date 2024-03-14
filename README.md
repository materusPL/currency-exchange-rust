# Currency Exchange
Currency exchange app using ExchangeRate-API.\
To use, need ExchangeRate-API Key which is obtainable [here](https://www.exchangerate-api.com/)
## Usage
Before using get your API-KEY and set it up with command or by using interactive mode
```
currency-exchange --set-api-key <API_KEY>
# or
currency-exchange --interactive
```
More information on usage:
```
Usage: currency-exchange [OPTIONS] [Currency input] [Currency target] [Amount]

Arguments:
  [Currency input]   Currency code to exchange from
  [Currency target]  Currency code to exchange to
  [Amount]           Currency amount to exchange

Options:
  -k, --set-api-key <API_KEY>  Set api key
  -r, --recreate-cache         Recrate cache
  -i, --interactive            Interactive mode
  -l, --list                   List currencies
  -L, --list-rates <currency>  List exchange rate for currency
  -h, --help                   Print help
```
# Build
Needs rust and cargo, build tested on rust v1.76.0\
To build run command
```
cargo build -r
```

