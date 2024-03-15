use std::{
    env::{temp_dir, var_os},
    path::PathBuf,
    process::exit,
};

pub const CACHE_LOCATION_ENV_NAME: &str = "CURRENCY_CACHE";
pub const REST_ENDPOINT: &str = "https://v6.exchangerate-api.com/v6/";
pub const REST_ENDPOINT_ENV_NAME: &str = "CURRENCY_ENDPOINT";

pub fn get_endpoint() -> String {
    let ret: String;
    match var_os(REST_ENDPOINT_ENV_NAME) {
        Some(val) => ret = val.to_str().unwrap().to_string(),
        None => ret = REST_ENDPOINT.to_string(),
    }

    ret
}
pub fn get_cache_path() -> PathBuf {
    let mut path: PathBuf = PathBuf::new();
    match var_os(CACHE_LOCATION_ENV_NAME) {
        Some(val) => path.push(val),
        None => {
            path.push(temp_dir());
            path.push("currencyCache.db");
        }
    }

    path
}
pub fn get_current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
