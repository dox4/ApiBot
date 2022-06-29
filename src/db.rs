use std::collections::HashMap;
use std::ops::RangeBounds;

use reqwest::Request;
use rusqlite::{self, Connection, Error};

use crate::http;
use crate::time;

pub(crate) const DB_PATH: &str = "~/.apibot/";
pub(crate) const DB_NAME: &str = "apibot_db";

const _CREATE_TABLE_VERSION: &str = "CREATE TABLE ab_version IF NOT EXISTS (
    version TEXT NOT NULL,
    created_at TEXT NOT NULL,
    deleted_at TEXT NOT NULL,
);";

const _CREATE_TABLE_GROUP: &str = "CREATE TABLE ab_group IF NOT EXISTS (
    name TEXT NOT NULL,
    description TEXT NULL,
    created_at TEXT NOT NULL,
    deleted_at TEXT NULL,
);";

const _CREATE_TABLE_REQUEST: &str = "CREATE TABLE ab_request IF NOT EXISTS (
    id INTEGER PRIMARY KEY,
    group TEXT NOT NULL,
    version REAL NOT NULL,
    url TEXT NOT NULL,
    header TEXT NOT NULL,
    body TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NULL,
    deleted_at TEXT NULL,
);";

const _CREATE_TABLE_RESPONSE: &str = "CREATE TABLE ab_response IF NOT EXISTS (
    id INTEGER PRIMARY KEY,
    request_id INTEGER NOT NULL,
    status_code INTEGER NOT NULL,
    header TEXT NOT NULL,
    body TEXT NOT NULL,
    received_at TEXT NOT NULL,
    deleted_at TEXT NULL,
);";

const _CREATE_TABLE_SCRIPT: &str = "CREATE TABLE ab_script IF NOT EXISTS (
    id INTEGER PRIMARY KEY,
    group TEXT NOT NULL,
    path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT NULL,
);";

const _CHECK_IF_DEFAULT_GROUP_EXISTS: &str = "SELECT * FROM ab_group WHERE name = 'default';";
const _INSERT_DEFAULT_GROUP: &str =
    "INSERT INTO ab_group value ('default', 'the default group of ApiBot', ?1, NULL);";

pub(crate) fn init() -> Result<(), Error> {
    // assume the DB_PATH already exists;
    let db = format!("{}{}", DB_PATH, DB_NAME);
    let conn = Connection::open(db)?;
    conn.execute(_CREATE_TABLE_VERSION, [])?;
    conn.execute(_CREATE_TABLE_GROUP, [])?;
    conn.execute(_CREATE_TABLE_REQUEST, [])?;
    conn.execute(_CREATE_TABLE_RESPONSE, [])?;
    conn.execute(_CREATE_TABLE_SCRIPT, [])?;
    // check if default group exists
    let mut stmt = conn.prepare(_CHECK_IF_DEFAULT_GROUP_EXISTS)?;
    let mut rows = stmt.query([])?;
    if let Ok(None) = rows.next() {
        println!("insert the default group...");
        conn.execute(_INSERT_DEFAULT_GROUP, &[&time::now()])?;
    }
    Ok(())
}

pub(crate) fn store_request(request: Request, group: String) {
    let db = format!("{}{}", DB_PATH, DB_NAME);
    let conn = Connection::open(db).unwrap();
    let version = http::from_http_version(request.version()).to_string();
    let url = request.url().to_string();
    let headers = request.headers();
    let map: HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect();
    let header = serde_json::to_string(&map).unwrap().to_string();
    let body = if let Some(body) = request.body() {
        unsafe { String::from_utf8_unchecked(body.as_bytes().unwrap().to_vec()) }
    } else {
        "".to_string()
    };
    conn.execute(
    "INSERT INTO ab_request (group, version, url, header, body, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        [group, version, url, header, "".to_string(), time::now()],
    )
    .unwrap();
}
