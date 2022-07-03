use clap::Result;
use http::{self, HeaderMap};

use reqwest::blocking::{Client, Request};
use reqwest::header::{HeaderName, HeaderValue};
use std::cell::RefCell;

use crate::command::{RetryOptions, SendOptions};
use crate::db;
use crate::model::RequestTable;

pub(crate) type Method = http::Method;
pub(crate) type Version = http::Version;

pub(crate) fn from_http_version(version: Version) -> String {
    match version {
        Version::HTTP_09 => "0.9".to_string(),
        Version::HTTP_10 => "1.0".to_string(),
        Version::HTTP_11 => "1.1".to_string(),
        Version::HTTP_2 => "2.0".to_string(),
        Version::HTTP_3 => "3.0".to_string(),
        _ => panic!("unrecognized http version."),
    }
}

pub(crate) fn make_request(
    url: String,
    method: Method,
    headers: Vec<(HeaderName, HeaderValue)>,
    http_version: Version,
    data: Option<String>,
) -> Result<Request, Box<dyn std::error::Error>> {
    let url = if !url.starts_with("http") {
        format!("http://{}", url)
    } else {
        url
    };
    let mut hm = HeaderMap::new();
    for header in headers {
        hm.insert(header.0, header.1);
    }
    let client = Client::new();
    let builder = client
        .request(method, url)
        .headers(hm)
        .version(http_version);
    match data {
        Some(data) => Ok(builder.body(data).build()?),
        None => Ok(builder.build()?),
    }
}

pub(crate) fn send(options: crate::command::SendOptions) -> Result<(), Box<dyn std::error::Error>> {
    let SendOptions {
        url,
        method,
        headers,
        http,
        data,
    } = options;
    let request = make_request(url, method, headers, http, data)?;
    let request_id = db::store_request(&request, "default".to_string());
    execute_request(request_id, request);
    Ok(())
}

pub(crate) fn execute_request(request_id: i64, request: Request)  {
    let client = reqwest::blocking::Client::new();
    let resp = client.execute(request).unwrap();
    let resp = RefCell::new(resp);
    let body = db::store_response(request_id, resp.borrow_mut(), "default".to_string());
    let j = serde_json::from_str::<serde_json::Value>(&body).unwrap();
    println!("{}", serde_json::to_string_pretty(&j).unwrap());
}


pub(crate) fn retry(options: RetryOptions) {
    let RetryOptions { request_id } = options;
    let req: RequestTable = db::retrive_resource_by_id(request_id);
    execute_request(request_id, req.to_request());
}
