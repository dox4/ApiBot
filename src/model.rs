use std::str::FromStr;

use http::{header::HeaderName, HeaderValue};
use reqwest::blocking::Request;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    command,
    http::{make_request, Method},
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct RequestTable {
    pub(crate) id: i64,
    pub(crate) created_at: String,
    pub(crate) updated_at: Option<String>,
    pub(crate) deleted_at: Option<String>,
    pub(crate) namespace: String,
    pub(crate) method: String,
    pub(crate) version: String,
    pub(crate) url: String,
    pub(crate) header: String,
    pub(crate) body: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ResponseTable {
    pub(crate) id: i64,
    pub(crate) namespace: String,
    pub(crate) request_id: i64,
    pub(crate) status_code: i32,
    pub(crate) header: String,
    pub(crate) body: String,
    pub(crate) received_at: String,
    pub(crate) deleted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ScriptTable {
    pub(crate) id: i64,
    pub(crate) created_at: String,
    pub(crate) updated_at: Option<String>,
    pub(crate) deleted_at: Option<String>,
    pub(crate) namespace: String,
    pub(crate) path: String,
}

impl RequestTable {
    pub(crate) fn to_request(self) -> Request {
        let url = self.url;
        let method = Method::from_bytes(self.method.as_bytes()).unwrap();
        let mut headers = Vec::new();
        let value = serde_json::from_str::<Value>(&self.header.as_str()).unwrap();
        for (k, v) in value.as_object().unwrap() {
            headers.push((
                HeaderName::from_str(k.as_str()).unwrap(),
                HeaderValue::from_str(v.as_str().unwrap()).unwrap(),
            ));
        }
        let http_version = command::parse_http_version(&self.version).unwrap();
        make_request(url, method, headers, http_version, Some(self.body)).unwrap()
    }
}

pub(crate) trait SQLiteTable {
    fn table_name() -> &'static str;
    fn retrive_stmt(limit: u32) -> String {
        format!(
            "SELECT * FROM {} WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT {}",
            Self::table_name(),
            limit
        )
    }
    fn retrive_by_id(id: i64) -> String {
        format!("SELECT * FROM {} WHERE id = {}", Self::table_name(), id)
    }
}

impl SQLiteTable for RequestTable {
    fn table_name() -> &'static str {
        "apibot_request"
    }
}

impl SQLiteTable for ResponseTable {
    fn table_name() -> &'static str {
        "apibot_response"
    }

    fn retrive_stmt(limit: u32) -> String {
        format!(
            "SELECT * FROM {} ORDER BY received_at DESC LIMIT {}",
            Self::table_name(),
            limit
        )
    }
}

impl SQLiteTable for ScriptTable {
    fn table_name() -> &'static str {
        "apibot_script"
    }
}
