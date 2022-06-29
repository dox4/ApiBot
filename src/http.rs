use std::collections::HashMap;

use http::{self, HeaderMap, Version};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Request,
};

pub(crate) type ApiBotHeaderName = HeaderName;

pub(crate) fn to_http_version(version: f64) -> Version {
    match version {
        0.9 => Version::HTTP_09,
        1.0 => Version::HTTP_10,
        1.1 => Version::HTTP_11,
        2.0 => Version::HTTP_2,
        3.0 => Version::HTTP_3,
        _ => panic!("bala bala"),
    }
}

pub(crate) fn from_http_version(version: Version) -> f64 {
    match version {
        Version::HTTP_09 => 0.9,
        Version::HTTP_10 => 1.0,
        Version::HTTP_11 => 1.1,
        Version::HTTP_2 => 2.0,
        Version::HTTP_3 => 3.0,
        _ => panic!("unrecognized http version.")
    }
}

pub(crate) fn make_request(
    url: String,
    method: Method,
    headers: Vec<(HeaderName, HeaderValue)>,
    http: f64,
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
    let client = reqwest::Client::new();
    let builder = client
        .request(method, url)
        .headers(hm)
        .version(to_http_version(http));
    if let Some(data) = data {
        match serde_json::from_slice::<HashMap<String, serde_json::Value>>(data.as_bytes()) {
            Ok(json) => Ok(builder.json(&json).build()?),
            Err(_) => Ok(builder.body(data).build()?),
        }
    } else {
        Ok(builder.build()?)
    }
}

pub(crate) type Method = http::Method;
pub(crate) async fn send(
    url: String,
    method: Method,
    headers: Vec<(HeaderName, HeaderValue)>,
    http: f64,
    data: Option<String>,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let url = if !url.starts_with("http") {
        format!("http://{}", url)
    } else {
        url
    };
    let mut hm = HeaderMap::new();
    for header in headers {
        hm.insert(header.0, header.1);
    }
    let client = reqwest::Client::new();
    let builder = client.request(method, url).headers(hm);
    if let Some(data) = data {
        match serde_json::from_slice::<HashMap<String, serde_json::Value>>(data.as_bytes()) {
            Ok(json) => Ok(builder.json(&json).send().await?),
            Err(_) => Ok(builder.body(data).send().await?),
        }
    } else {
        Ok(builder.send().await?)
    }
}
