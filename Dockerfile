FROM rust:1.76.0

WORKDIR /usr/src/currency-exchange

COPY . .

RUN cargo build -r

ENTRYPOINT  [ "./target/release/currency-exchange" ]
