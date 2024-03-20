FROM rust:1.76.0

ENV CURRENCY_CACHE="/tmp/docker_currency_cache.db"

WORKDIR /usr/src/currency-exchange

COPY . .

RUN cargo build -r

ENTRYPOINT  [ "./target/release/currency-exchange" ]
