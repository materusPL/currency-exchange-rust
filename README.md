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
  -r, --recreate-cache         Recreate cache
  -i, --interactive            Interactive mode
  -l, --list                   List currencies
  -L, --list-rates <currency>  List exchange rate for currency
  -h, --help                   Print help
```
Cache and api key is stored by default in `<XDG_CACHE_HOME>/currencyCache.db` or `<TMPDIR>/currencyCache.db` if `XDG_CACHE_HOME` is not set. This location and filename can be overriden by setting up `CURRENCY_CACHE` env variable.

## Build
Needs rust and cargo, build tested on rust v1.76.0\
To build run command
```
cargo build -r
```

## Docker
[![status](https://github.com/materusPL/currency-exchange-rust/actions/workflows/main.yml/badge.svg)](https://hub.docker.com/repository/docker/materus/currency-exchange)

To run with docker use:
```
docker run -it --rm -v /tmp:/tmp:rw materus/currency-exchange:latest <ARGUMENTS>
```

Dockerfile sets up `CURRENCY_CACHE` to `/tmp/docker_currency_cache.db`, to keep cache u should mount this file or tmp directory for example with `-v /tmp:/tmp:rw` in docker run cmdline.


