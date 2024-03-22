use std::collections::HashMap;

use crate::cache::{self, get_api_key};
use crate::config::get_endpoint;
use serde::Deserialize;
#[derive(PartialEq)]
pub enum Status {
    OK,
    INVALID,
    LIMIT,
    ERROR,
}

#[derive(Deserialize)]
struct CurrencyCodes {
    supported_codes: Vec<[String; 2]>,
}
#[derive(Deserialize)]
struct ConversionRates {
    base_code: String,
    time_next_update_unix: u64,

    conversion_rates: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct Err {
    #[serde(rename = "error-type")]
    error_type: String,
}
#[cfg(test)]
fn mock_get_rates(code: &String) -> Result<Status, reqwest::Error> {
    let response: ConversionRates = match code.as_str() {
        "PLN" => serde_json::from_str(include_str!(concat!(
            ".",
            crate::main_separator!(),
            "mock_data",
            crate::main_separator!(),
            "PLN.json"
        )))
        .expect("Error when deserializng"),
        "EUR" => serde_json::from_str(include_str!(concat!(
            ".",
            crate::main_separator!(),
            "mock_data",
            crate::main_separator!(),
            "EUR.json"
        )))
        .expect("Error when deserializng"),
        _ => {
            panic!("Unknown code")
        }
    };
    cache::add_rates(
        response.time_next_update_unix,
        &response.base_code,
        &response.conversion_rates,
    )
    .expect("Error while caching response");
    Ok(Status::OK)
}
#[cfg(test)]
pub fn mock_get_currencies() -> Result<Status, reqwest::Error> {
    let codes: CurrencyCodes = serde_json::from_str(include_str!(concat!(
        ".",
        crate::main_separator!(),
        "mock_data",
        crate::main_separator!(),
        "codes.json"
    )))
    .expect("Error when deserializng");
    for code in codes.supported_codes {
        cache::add_code(code).expect("Error when adding code to cache");
    }
    return Ok(Status::OK);
}

pub fn get_rates(code: &String) -> Result<Status, reqwest::Error> {
    if cfg!(test) {
        #[cfg(test)]
        return mock_get_rates(code);
    }
    let response = reqwest::blocking::get(format!(
        "{}{}{}{}",
        get_endpoint(),
        get_api_key().expect("Error when getting api key from cache"),
        "/latest/",
        code.to_uppercase()
    ))?;
    if response.status().is_success() {
        let response: ConversionRates =
            serde_json::from_str(&response.text()?).expect("Error when deserializng");
        cache::add_rates(
            response.time_next_update_unix,
            &response.base_code,
            &response.conversion_rates,
        )
        .expect("Error while caching response");
        return Ok(Status::OK);
    } else {
        let err: Err = serde_json::from_str(&response.text()?).expect("Error when deserializng");
        if err.error_type == "invalid-key" {
            return Ok(Status::INVALID);
        } else if err.error_type == "quota-reached" {
            return Ok(Status::LIMIT);
        }
    }

    Ok(Status::ERROR)
}
pub fn get_currencies() -> Result<Status, reqwest::Error> {
    if cfg!(test) {
        #[cfg(test)]
        return mock_get_currencies();
    }
    let response = reqwest::blocking::get(format!(
        "{}{}{}",
        get_endpoint(),
        get_api_key().expect("Error when getting api key from cache"),
        "/codes"
    ))?;
    if response.status().is_success() {
        let codes: CurrencyCodes =
            serde_json::from_str(&response.text()?).expect("Error when deserializng");
        for code in codes.supported_codes {
            cache::add_code(code).expect("Error when adding code to cache");
        }
        return Ok(Status::OK);
    } else {
        let err: Err = serde_json::from_str(&response.text()?).expect("Error when deserializng");
        if err.error_type == "invalid-key" {
            return Ok(Status::INVALID);
        } else if err.error_type == "quota-reached" {
            return Ok(Status::LIMIT);
        }
    }

    Ok(Status::ERROR)
}
