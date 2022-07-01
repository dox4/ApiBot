use serde_derive::{Deserialize, Serialize};

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
    namespace: String,
    path: String,
}
