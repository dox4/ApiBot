use std::collections::HashMap;
use std::io::Read;

use reqwest::blocking::Request;
use reqwest::blocking::Response;
use rusqlite::{self, Connection, Error};

use crate::http;
use crate::time;
use crate::util;
pub(crate) const DB_NAME: &str = "apibot_db";

const _CREATE_TABLE_VERSION: &str = "CREATE TABLE IF NOT EXISTS apibot_version (
    version TEXT NOT NULL,
    created_at TEXT NOT NULL,
    deleted_at TEXT NOT NULL
);";

const _CREATE_TABLE_NAMESPACE: &str = "CREATE TABLE IF NOT EXISTS apibot_namespace (
    name TEXT NOT NULL,
    description TEXT NULL,
    created_at TEXT NOT NULL,
    deleted_at TEXT NULL
);";

const _CREATE_TABLE_REQUEST: &str = "CREATE TABLE IF NOT EXISTS apibot_request (
    id INTEGER PRIMARY KEY,
    namespace TEXT NOT NULL,
    version TEXT NOT NULL,
    url TEXT NOT NULL,
    header TEXT NOT NULL,
    body TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NULL,
    deleted_at TEXT NULL
);";

const _CREATE_TABLE_RESPONSE: &str = "CREATE TABLE IF NOT EXISTS apibot_response (
    id INTEGER PRIMARY KEY,
    request_id INTEGER NOT NULL,
    status_code INTEGER NOT NULL,
    header TEXT NOT NULL,
    body TEXT NOT NULL,
    received_at TEXT NOT NULL,
    deleted_at TEXT NULL
);";

const _CREATE_TABLE_SCRIPT: &str = "CREATE TABLE IF NOT EXISTS apibot_script (
    id INTEGER PRIMARY KEY,
    namespace TEXT NOT NULL,
    path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT NULL
);";

const _CHECK_IF_DEFAULT_GROUP_EXISTS: &str = "SELECT * FROM apibot_namespace WHERE name = 'default';";
const _INSERT_DEFAULT_GROUP: &str =
    "INSERT INTO apibot_namespace VALUES ('default', 'the default group of ApiBot', ?1, NULL);";

fn path_to_db() -> String {
    util::app_home_dir()
        .join(DB_NAME)
        .to_str()
        .unwrap()
        .to_string()
}

pub(crate) fn init() -> Result<(), Error> {
    // assume the DB_PATH already exists;
    let db = path_to_db();
    println!("{}", db);
    let conn = Connection::open(db)?;
    conn.execute(_CREATE_TABLE_VERSION, []).expect(_CREATE_TABLE_VERSION);
    conn.execute(_CREATE_TABLE_NAMESPACE, []).expect(_CREATE_TABLE_NAMESPACE);
    conn.execute(_CREATE_TABLE_REQUEST, []).expect(_CREATE_TABLE_REQUEST);
    conn.execute(_CREATE_TABLE_RESPONSE, []).expect(_CREATE_TABLE_RESPONSE);
    conn.execute(_CREATE_TABLE_SCRIPT, []).expect(_CREATE_TABLE_SCRIPT);
    // check if default group exists
    let mut stmt = conn.prepare(_CHECK_IF_DEFAULT_GROUP_EXISTS)?;
    let mut rows = stmt.query([])?;
    if let Ok(None) = rows.next() {
        println!("insert the default group...");
        conn.execute(_INSERT_DEFAULT_GROUP, &[&time::now()])?;
    }
    Ok(())
}

fn headers_to_string(headers: &reqwest::header::HeaderMap) -> String {
    let map: HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect();
    serde_json::to_string(&map).unwrap().to_string()
}

pub(crate) fn store_request(request: &Request, group: String) -> i64 {
    let db = path_to_db();
    let conn = Connection::open(db).unwrap();
    let version = http::from_http_version(request.version()).to_string();
    let url = request.url().to_string();
    let headers = headers_to_string(&request.headers());
    let body = if let Some(body) = request.body() {
        unsafe { String::from_utf8_unchecked(body.as_bytes().unwrap().to_vec()) }
    } else {
        "".to_string()
    };
    conn.execute(
    "INSERT INTO apibot_request (namespace, version, url, header, body, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        [group, version, url, headers, body, time::now()],
    )
    .unwrap();
    conn.last_insert_rowid()
}

pub(crate) fn store_response(request_id: i64, resp: &Response, group: String) {
    let db = path_to_db();
    let conn = Connection::open(db).unwrap();
    let request_id = request_id.to_string();
    let status_code = resp.status().to_string();
    let headers = headers_to_string(&resp.headers());
    // let body = if resp
    //     .headers()
    //     .get(reqwest::header::CONTENT_TYPE)
    //     .unwrap_or(&reqwest::header::HeaderValue::from_str("application/json").unwrap())
    //     .to_str()
    //     .unwrap()
    //     .contains("application/json")
    // {
    //     serde_json::to_string(resp.json::<HashMap<String, serde_json::Value>>().await.unwrap()).unwrap()
    // } else {
    //     resp.text().await.unwrap().to_string()
    // };
    //
    //  let body = match resp.json::<HashMap<String, serde_json::Value>>().await {
    //      Ok(_) => todo!(),
    //      Err(_) => todo!(),
    //  };
    //  conn.execute(
    //      "INSERT INTO apibot_response (request_id, status_code, header, body, received_at) VALUES ();",
    //      [request_id, status_code, headers, body, time::now()],
    //  )
    //  .unwrap();
}
