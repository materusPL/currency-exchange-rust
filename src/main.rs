use std::io::Write;

use crate::{
    cache::{create_cache, set_api_key},
    requests::get_currencies,
};
use cache::check_code;
use clap::Parser;
use exchange::convert_value;
mod cache;
mod config;
mod exchange;
mod requests;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Currency code to exchange from
    #[arg(value_names = ["Currency input"])]
    currency_from: Option<String>,
    /// Currency code to exchange to
    #[arg(value_names = ["Currency target"])]
    currency_to: Option<String>,
    /// Currency amount to exchange
    #[arg(value_names = ["Amount"])]
    value: Option<String>,

    /// Set api key
    #[arg(short = 'k', long = "set-api-key")]
    api_key: Option<String>,
    /// Recreate cache
    #[arg(short = 'r', long = "recreate-cache")]
    recreate_cache: bool,
    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// List currencies
    #[arg(short, long)]
    list: bool,

    /// List exchange rate for currency
    #[arg(short = 'L', long = "list-rates", value_names = ["currency"])]
    list_rates: Option<String>,
}
async fn setup_key(key: String) -> Result<bool, Box<dyn std::error::Error>> {
    set_api_key(key)?;
    let status = get_currencies().await?;
    if status == requests::Status::INVALID {
        set_api_key("".to_string())?;
        println!("Api Key is invalid");
        return Ok(false);
    } else if status == requests::Status::LIMIT {
        set_api_key("".to_string())?;
        println!("Can't set up API key due to exceeded API limit");
        return Ok(false);
    } else if status == requests::Status::ERROR {
        set_api_key("".to_string())?;
        println!("Can't set up API key due to unknown error");
        return Ok(false);
    }
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let all_args =
        args.currency_from.is_some() && args.currency_to.is_some() && args.value.is_some();
    let wrong_args =
        args.currency_from.is_some() && (args.currency_to.is_none() || args.value.is_none());

    if args.interactive && (all_args || wrong_args) {
        panic!("Do not provide codes and value with --interactive")
    }
    if args.recreate_cache || !config::get_cache_path().exists() {
        create_cache()?;
    }
    match args.api_key {
        None => {}
        Some(key) => {
            let res = setup_key(key)
                .await
                .expect("Unknown error while setting up key");
            if !res {
                return Ok(());
            }
        }
    }
    if !args.interactive {
        if !(cache::get_api_key()
            .expect("Error while getting api key")
            .len()
            > 0)
        {
            panic!("API Key is not set up!");
        }
        if args.list {
            let currencies = cache::list_currencies()?;
            for currency in currencies {
                println!("{} - {}", currency[0], currency[1]);
            }
        } else if args.list_rates.is_some() {
            let code = args.list_rates.unwrap().clone();
            let check = check_code(&code)?;
            if !check {
                panic!("Code {} not found", code);
            }
            exchange::update_rate(&code).await;
            let rates = cache::list_rates(&code)?;
            for rate in rates {
                println!("{} to {} rate: {}", code, rate[0], rate[1]);
            }
        } else if wrong_args {
            panic!("Not all args specified, provide 'currency from', 'currency to' and 'amount'");
        } else if all_args {
            convert_value(
                &args.currency_from.unwrap().to_uppercase(),
                &args.currency_to.unwrap().to_uppercase(),
                &args.value.unwrap(),
            )
            .await
        }
    } else {
        interactive().await?;
    }
    Ok(())
}
async fn interactive() -> Result<(), Box<dyn std::error::Error>> {
    let mut key_setup = cache::get_api_key()
        .expect("Error while getting api key")
        .len()
        > 0;
    while !key_setup {
        let mut key_string = String::new();
        print!("Please enter API Key: ");
        std::io::stdout().flush()?;
        std::io::stdin()
            .read_line(&mut key_string)
            .expect("Did not enter a correct string");
        setup_key(key_string.trim().to_string())
            .await
            .expect("Unknown error while setting up key");
        key_setup = cache::get_api_key()
            .expect("Error while getting api key")
            .len()
            > 0;
    }

    let mut code_from: String = String::new();
    let mut code_to: String = String::new();
    let mut amount: String = String::new();

    let mut code_from_check = false;
    let mut code_to_check = false;
    let mut amount_check = false;

    while !code_from_check {
        code_from = String::new();
        print!("Please enter code of input currency: ");
        std::io::stdout().flush()?;
        std::io::stdin()
            .read_line(&mut code_from)
            .expect("Did not enter a correct string");
        code_from = code_from.trim().to_uppercase().to_string();
        code_from_check = cache::check_code(&code_from)?;
        if !code_from_check {
            println!("Code {} is unknown", code_from);
        }
    }
    while !code_to_check {
        code_to = String::new();
        print!("Please enter code of output currency: ");
        std::io::stdout().flush()?;
        std::io::stdin()
            .read_line(&mut code_to)
            .expect("Did not enter a correct string");
        code_to = code_to.trim().to_uppercase().to_string();
        code_to_check = cache::check_code(&code_to)?;
        if !code_to_check {
            println!("Code {} is unknown", code_to);
        }
    }

    while !amount_check {
        amount = String::new();
        print!("Please enter amount of input currency: ");
        std::io::stdout().flush()?;
        std::io::stdin()
            .read_line(&mut amount)
            .expect("Did not enter a correct string");
        amount = amount.trim().to_string();
        if amount.parse::<f64>().is_err() {
            println!("{} is not a number!", amount)
        } else {
            amount_check = true
        }
    }
    convert_value(&code_from, &code_to, &amount).await;

    Ok(())
}
