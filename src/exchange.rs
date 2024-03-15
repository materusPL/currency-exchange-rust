use crate::*;
use rust_decimal::prelude::*;
use rusty_money::{iso::find, ExchangeRate, Money};
pub async fn update_rate(code: &String) {
    if cache::get_next_update(code).expect("Error getting next update time from cache")
        <= config::get_current_time()
    {
        let status = requests::get_rates(code)
            .await
            .expect("Error while fetching rates");
        if status == requests::Status::INVALID {
            panic!("Invalid api key when getting rates")
        } else if status == requests::Status::LIMIT {
            panic!("Exceeded API limit when getting rates")
        } else if status == requests::Status::ERROR {
            panic!("Unknown error when getting rates")
        }
    }
}
pub async fn get_rate(code_from: &String, code_to: &String) -> String {
    if !cache::check_code(code_from).expect("Error on getting code status") {
        panic!("Code {} doesn't exists, use correct code!", code_from);
    }
    if !cache::check_code(code_to).expect("Error on getting code status") {
        panic!("Code {} doesn't exists, use correct code!", code_to);
    }
    if (!cache::check_exchange(code_from, code_to).expect("Error on getting exchange status"))
        || (cache::get_next_update(code_from).expect("Error getting next update time from cache")
            <= config::get_current_time())
    {
        update_rate(code_from).await;
    }
    cache::get_rate(code_from, code_to).expect("Error when getting cached rate")
}

pub async fn convert_value(code_from: &String, code_to: &String, value: &String) {
    if value.parse::<f64>().is_err() {
        panic!("{} is not a number!", value);
    }
    let text_rate = get_rate(code_from, code_to).await;
    let from_currency = find(code_from);
    if from_currency.is_none() {
        panic!("{} not found in ISO formats", code_from);
    }
    let to_currency = find(code_to);
    if to_currency.is_none() {
        panic!("{} not found in ISO formats", code_to);
    }

    let rate = Decimal::from_str(&text_rate).unwrap();
    let dec_amount = Decimal::from_str(&value).unwrap();
    let from_money = Money::from_decimal(dec_amount, from_currency.unwrap());
    println!("Input: {}", from_money.to_string());
    if code_from != code_to {
        let ex = ExchangeRate::new(from_currency.unwrap(), to_currency.unwrap(), rate).unwrap();
        let result = ex.convert(from_money).expect("Error while conversion");
        println!("Equals: {}", result.to_string())
    } else {
        println!("{}", from_money.to_string())
    }
    println!("Exchange rate: {}", text_rate);
}
