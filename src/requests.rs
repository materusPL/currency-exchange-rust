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

pub async fn get_rates(code: &String) -> Result<Status, reqwest::Error> {
    let response = reqwest::get(format!(
        "{}{}{}{}",
        get_endpoint(),
        get_api_key().expect("Error when getting api key from cache"),
        "/latest/",
        code.to_uppercase()
    ))
    .await?;
    if response.status().is_success() {
        let response: ConversionRates =
            serde_json::from_str(&response.text().await?).expect("Error when deserializng");
        cache::add_rates(
            response.time_next_update_unix,
            &response.base_code,
            &response.conversion_rates,
        )
        .expect("Error while caching response");
        return Ok(Status::OK);
    } else {
        let err: Err =
            serde_json::from_str(&response.text().await?).expect("Error when deserializng");
        if err.error_type == "invalid-key" {
            return Ok(Status::INVALID);
        } else if err.error_type == "quota-reached" {
            return Ok(Status::LIMIT);
        }
    }

    Ok(Status::ERROR)
}
pub async fn get_currencies() -> Result<Status, reqwest::Error> {
    let response = reqwest::get(format!(
        "{}{}{}",
        get_endpoint(),
        get_api_key().expect("Error when getting api key from cache"),
        "/codes"
    ))
    .await?;
    if response.status().is_success() {
        let codes: CurrencyCodes =
            serde_json::from_str(&response.text().await?).expect("Error when deserializng");
        for code in codes.supported_codes {
            cache::add_code(code).expect("Error when adding code to cache");
        }
        return Ok(Status::OK);
    } else {
        let err: Err =
            serde_json::from_str(&response.text().await?).expect("Error when deserializng");
        if err.error_type == "invalid-key" {
            return Ok(Status::INVALID);
        } else if err.error_type == "quota-reached" {
            return Ok(Status::LIMIT);
        }
    }

    Ok(Status::ERROR)
}
