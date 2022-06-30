use http::{self, HeaderMap, Version};

use reqwest::blocking::{Client, Request, Response};
use reqwest::header::{HeaderName, HeaderValue};
use std::cell::RefCell;
use std::io::Read;

use crate::db;

pub(crate) type Method = http::Method;

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
        _ => panic!("unrecognized http version."),
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
    let client = Client::new();
    let builder = client
        .request(method, url)
        .headers(hm)
        .version(to_http_version(http));
    // if let Some(data) = data {
    //     match serde_json::from_slice::<HashMap<String, serde_json::Value>>(data.as_bytes()) {
    //         Ok(json) => Ok(builder.json(&json).build()?),
    //         Err(_) => Ok(builder.body(data).build()?),
    //     }
    // } else {
    //     Ok(builder.build()?)
    // }
    Ok(builder.build()?)
}

pub(crate) fn send(options: crate::command::SendOptions) -> Result<(), Box<dyn std::error::Error>> {
    let crate::command::SendOptions {
        url,
        method,
        headers,
        http,
        data,
    } = options;
    let request = make_request(url, method, headers, http, data)?;
    let request_id = db::store_request(&request, "default".to_string());
    let client = reqwest::blocking::Client::new();
    let resp = client.execute(request).unwrap();
    db::store_response(request_id, &resp, "default".to_string());
    let resp = RefCell::new(resp);
    let mut buf: Vec<u8> = Vec::new();
    let size = resp.borrow_mut().read_to_end(&mut buf).unwrap();
    let j = serde_json::from_slice::<serde_json::Value>(&buf[..]).unwrap();
    println!("{}", serde_json::to_string_pretty(&j).unwrap());
    Ok(())
}
