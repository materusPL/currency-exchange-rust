use crate::{cache::get_api_key, *};
use std::sync::Once;

static INIT: Once = Once::new();

fn setup_test() {
    INIT.call_once(|| {
        let mut path: std::path::PathBuf = std::path::PathBuf::new();
        path.push(std::env::temp_dir());
        path.push("testCurrencyCache.db");

        std::env::set_var(config::CACHE_LOCATION_ENV_NAME, &path);
        if path.exists() {
            std::fs::remove_file(path).expect("Something went wrong when removing test cache");
        }
        cache::create_cache().expect("Something went wrong when creating test cache");
        cache::set_api_key("testKey".to_string())
            .expect("Something went wrong when setting api key");
        cache::add_code(["PLN".to_string(), "Polish zloty".to_string()])
            .expect("Something went wrong when adding code");

        let mut rates: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        rates.insert("USD".to_string(), serde_json::json!(0.2546));
        rates.insert("EUR".to_string(), serde_json::json!(0.2325));
        cache::add_rates(99710201602, &"PLN".to_string(), &rates).expect("Error seting rates");
    });
}
#[test]
fn test_cache_get_api_key() {
    setup_test();

    assert_eq!(
        get_api_key().expect("Something went wrong when getting api key"),
        "testKey"
    );
}
#[test]
fn test_cache_check_code() {
    setup_test();

    assert!(cache::check_code(&"PLN".to_string()).expect("Something went wrong when getting code"));
}

#[test]
fn test_cache_get_rates() {
    setup_test();

    assert_eq!(
        cache::get_rate(&"PLN".to_string(), &"USD".to_string()).expect("Error getting rates"),
        "0.2546"
    );
    assert_eq!(
        cache::get_rate(&"PLN".to_string(), &"EUR".to_string()).expect("Error getting rates"),
        "0.2325"
    );
}

#[test]
fn test_cache_check_exchange() {
    setup_test();

    assert!(
        cache::check_exchange(&"PLN".to_string(), &"USD".to_string()).expect("Error while checking exchange")
    );
    assert!(
        cache::check_exchange(&"PLN".to_string(), &"EUR".to_string()).expect("Error while checking exchange")
    );
}

