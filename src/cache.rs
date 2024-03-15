use std::{
    collections::HashMap,
    fs::{metadata, remove_file},
};

use rusqlite::{Connection, Result};

use crate::config::get_cache_path;

const CANNOT_CLOSE_MSG: &str = "Couldn't close sqlite connection";

pub fn check_code(code: &String) -> Result<bool> {
    let conn = Connection::open(get_cache_path())?;
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT code FROM currencies WHERE currencies.code = UPPER($1))",
        [code],
        |row| row.get(0),
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(exists)
}

pub fn list_currencies() -> Result<Vec<[String; 2]>> {
    let conn = Connection::open(get_cache_path())?;
    let mut stmt = conn.prepare("SELECT code, text FROM currencies ORDER BY code")?;
    let ret = stmt
        .query_map([], |row| {
            let v: Result<[String; 2]> = Ok([row.get(0)?, row.get(1)?]);
            v
        })
        .expect("Error while listing currencies");

    let mut result: Vec<[String; 2]> = Vec::new();
    for code in ret {
        let i = code.unwrap();
        let z = [i[0].clone(), i[1].clone()];
        result.push(z);
    }
    stmt.finalize()?;
    conn.close().expect(CANNOT_CLOSE_MSG);
    Ok(result)
}

pub fn list_rates(code_from: &String) -> Result<Vec<[String; 2]>> {
    let conn = Connection::open(get_cache_path())?;
    let mut stmt = conn.prepare(
        "SELECT code_to, rate FROM exchange_rates WHERE code_from = $1 ORDER BY code_to",
    )?;
    let ret = stmt
        .query_map([code_from], |row| {
            let v: Result<[String; 2]> = Ok([row.get(0)?, row.get(1)?]);
            v
        })
        .expect("Error while listing rates");

    let mut result: Vec<[String; 2]> = Vec::new();
    for code in ret {
        let i = code.unwrap();
        let z = [i[0].clone(), i[1].clone()];
        result.push(z);
    }
    stmt.finalize()?;
    conn.close().expect(CANNOT_CLOSE_MSG);
    Ok(result)
}

pub fn check_exchange(code_from: &String, code_to: &String) -> Result<bool> {
    let conn = Connection::open(get_cache_path())?;
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT code_from, code_to 
            FROM exchange_rates 
            WHERE exchange_rates.code_from = UPPER($1) AND exchange_rates.code_to = UPPER($2))",
        [code_from, code_to],
        |row| row.get(0),
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(exists)
}

pub fn get_rate(code_from: &String, code_to: &String) -> Result<String> {
    let conn = Connection::open(get_cache_path())?;
    let rate: String = conn.query_row(
        "SELECT rate 
            FROM exchange_rates 
            WHERE exchange_rates.code_from = UPPER($1) AND exchange_rates.code_to = UPPER($2)",
        [code_from, code_to],
        |row| row.get(0),
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(rate)
}
pub fn get_next_update(code: &String) -> Result<u64> {
    let conn = Connection::open(get_cache_path())?;
    let next_update: u64 = conn.query_row(
        "SELECT next_update FROM currencies WHERE currencies.code = UPPER($1)",
        [code],
        |row| row.get(0),
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(next_update)
}

pub fn add_rates(
    next_update: u64,
    code_from: &String,
    rates: &HashMap<String, serde_json::Value>,
) -> Result<()> {
    let conn = Connection::open(get_cache_path())?;

    for (code_to, rate) in rates {
        conn.execute(
            "
    INSERT OR REPLACE INTO exchange_rates(code_from,code_to,rate)
    VALUES(UPPER($1),UPPER($2),$3)
    ",
            [code_from, code_to, &rate.to_string()],
        )?;
    }
    conn.execute(
        "
    UPDATE currencies
    SET next_update = $1
    WHERE currencies.code = UPPER($2)
    ",
        [&next_update.to_string(), code_from],
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);
    Ok(())
}

pub fn add_code(code: [String; 2]) -> Result<()> {
    let conn = Connection::open(get_cache_path())?;
    conn.execute(
        "
    INSERT OR IGNORE INTO currencies(code,text,next_update)
    VALUES(UPPER($1),$2,0)
    ",
        [code.get(0), code.get(1)],
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);
    Ok(())
}
pub fn get_api_key() -> Result<String> {
    let conn = Connection::open(get_cache_path())?;
    let api_key: String = conn.query_row(
        "SELECT value FROM config WHERE config.name = 'API_KEY'",
        [],
        |row| row.get(0),
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(api_key)
}
pub fn set_api_key(key: String) -> Result<()> {
    let conn = Connection::open(get_cache_path())?;
    conn.execute(
        "
    UPDATE config
    SET value = $1
    WHERE config.name = 'API_KEY'
    ",
        [key],
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(())
}

pub fn create_cache() -> Result<()> {
    let path = &get_cache_path();
    if path.is_dir() {
        panic!("Specified path cache path is dir, not file")
    }
    if path.exists() {
        match remove_file(path) {
            Ok(()) => (),
            Err(_e) => match metadata(path) {
                Ok(md) => {
                    if md.permissions().readonly() {
                        panic!("Can't modify file");
                    }
                }
                Err(_e) => panic!("Unknown error while trying to remove old database"),
            },
        }
    }
    let conn = Connection::open(path)?;

    conn.execute(
        "
    CREATE TABLE config (
        name   TEXT PRIMARY KEY,
        value  TEXT NOT NULL
    )",
        (),
    )?;

    conn.execute(
        "
    CREATE TABLE currencies (
        code   TEXT PRIMARY KEY,
        text   TEXT NOT NULL,
        next_update  TIME NOT NULL
    )",
        (),
    )?;

    conn.execute(
        "
    CREATE TABLE exchange_rates (
        code_from   TEXT NOT NULL,
        code_to   TEXT NOT NULL,
        rate    TEXT NOT NULL,
        PRIMARY KEY (code_from, code_to)
    )",
        (),
    )?;

    conn.execute(
        "
    INSERT INTO config (name, value) VALUES (
        'API_KEY',
        ''
    )
    ",
        (),
    )?;
    conn.close().expect(CANNOT_CLOSE_MSG);

    Ok(())
}
