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
Usage: currency-exchange [OPTIONS] [CURRENCY_FROM] [CURRENCY_TO] [VALUE]

Arguments:
  [CURRENCY_FROM]  Currency code to exchange from
  [CURRENCY_TO]    Currency code to exchange to
  [VALUE]          Currency amount to exchange

Options:
  -k, --set-api-key <API_KEY>    Set api key
  -r, --recreate-cache           Recrate cache
  -i, --interactive              Interactive mode
  -l, --list                     List currencies
  -L, --list-rates <LIST_RATES>  List exchange rate for currency
  -h, --help                     Print help

```
# Build
Needs rust and cargo, build tested on rust v1.76.0\
To build run command
```
cargo build -r
```

