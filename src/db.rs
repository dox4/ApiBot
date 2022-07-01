use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::collections::HashMap;
use std::io::Read;

use reqwest::blocking::Request;
use reqwest::blocking::Response;
use rusqlite::{self, Connection, Error};
use serde::de::DeserializeOwned;
use serde_rusqlite::from_rows;

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
    method TEXT NOT NULL,
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
    namespace TEXT NOT NULL,
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

const _CHECK_IF_DEFAULT_GROUP_EXISTS: &str =
    "SELECT * FROM apibot_namespace WHERE name = 'default';";
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
    let conn = Connection::open(db)?;
    conn.execute(_CREATE_TABLE_VERSION, [])
        .expect(_CREATE_TABLE_VERSION);
    conn.execute(_CREATE_TABLE_NAMESPACE, [])
        .expect(_CREATE_TABLE_NAMESPACE);
    conn.execute(_CREATE_TABLE_REQUEST, [])
        .expect(_CREATE_TABLE_REQUEST);
    conn.execute(_CREATE_TABLE_RESPONSE, [])
        .expect(_CREATE_TABLE_RESPONSE);
    conn.execute(_CREATE_TABLE_SCRIPT, [])
        .expect(_CREATE_TABLE_SCRIPT);
    // check if default group exists
    let mut stmt = conn.prepare(_CHECK_IF_DEFAULT_GROUP_EXISTS)?;
    let mut rows = stmt.query([])?;
    if let Ok(None) = rows.next() {
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
pub(crate) fn store_request(request: &Request, namespace: String) -> i64 {
    let db = path_to_db();
    let conn = Connection::open(db).unwrap();
    let version = http::from_http_version(request.version());
    let method = request.method().to_string();
    let url = request.url().to_string();
    let headers = headers_to_string(&request.headers());
    let body = if let Some(body) = request.body() {
        unsafe { String::from_utf8_unchecked(body.as_bytes().unwrap().to_vec()) }
    } else {
        "".to_string()
    };
    conn.execute(
    "INSERT INTO apibot_request (namespace, method, version, url, header, body, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
        [namespace, method, version, url, headers, body, time::now()],
    )
    .unwrap();
    conn.last_insert_rowid()
}

pub(crate) fn store_response(
    request_id: i64,
    mut resp: RefMut<Response>,
    namespace: String,
) -> String {
    let db = path_to_db();
    let conn = Connection::open(db).unwrap();
    let request_id = request_id.to_string();
    let status_code = resp.status().to_string();
    let headers = headers_to_string(&resp.headers());
    let mut buf: Vec<u8> = Vec::new();
    resp.borrow_mut().read_to_end(&mut buf).unwrap();
    let body = unsafe { String::from_utf8_unchecked(buf) };
    conn.execute(
        "INSERT INTO apibot_response (namespace, request_id, status_code, header, body, received_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        [namespace, request_id, status_code, headers, body.clone(), time::now()],
    )
    .unwrap();
    body
}

pub(crate) fn retrive_resource<T>(limit: u32) -> Vec<T>
where
    T: DeserializeOwned,
{
    let db = path_to_db();
    let conn = Connection::open(db).unwrap();
    let mut stmt = conn.prepare(format!("SELECT * FROM apibot_request WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT {}", limit).as_str()).unwrap();
    let res = from_rows::<T>(stmt.query([]).unwrap());
    res.map(|r| r.unwrap()).collect::<Vec<T>>()
}
