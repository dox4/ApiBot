use core::panic;
use std::error::Error;

use clap::{Parser, Subcommand};
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

use crate::db;
use crate::http::retry;
use crate::http::{self, Method};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) commands: SubCommand,
}

#[derive(Parser, Debug)]
pub(crate) struct SendOptions {
    #[clap(value_parser, help = "url to send request to.")]
    pub(crate) url: String,
    #[clap(
        short = 'X',
        long,
        value_parser,
        default_value = "GET",
        help = "HTTP request method, default is 'GET'."
    )]
    pub(crate) method: Method,
    #[clap(short = 'H',long, value_parser = parse_header, multiple = true, help = "headers of the HTTP request.")]
    pub(crate) headers: Vec<(HeaderName, HeaderValue)>,
    #[clap(long, default_value = "1.1", value_parser = parse_http_version, help = "http version, default is 1.1.")]
    pub(crate) http: crate::http::Version,
    #[clap(short, long, help = "data to send with the request.")]
    pub(crate) data: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum ResourceType {
    Request,
    Response,
    Script,
}

#[derive(Parser, Debug)]
pub(crate) struct ListOptions {
    #[clap(value_parser = parse_resource_type)]
    pub(crate) resource_type: ResourceType,
    #[clap(short, long, default_value = "10", value_parser)]
    pub(crate) limit: u32,
}

#[derive(Parser, Debug)]
pub(crate) struct RetryOptions {
    #[clap(value_parser)]
    pub(crate) request_id: i64,
}

#[derive(Parser, Debug)]
pub(crate) struct DescribeOptions {
    #[clap(value_parser = parse_resource_type)]
    pub(crate) resource_type: ResourceType,
    #[clap(value_parser)]
    pub(crate) request_id: i64,
}

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    Send {
        #[clap(flatten)]
        options: SendOptions,
    },
    List {
        #[clap(flatten)]
        options: ListOptions,
    },
    Retry {
        #[clap(flatten)]
        options: RetryOptions,
    },
    Describe {
        #[clap(flatten)]
        options: DescribeOptions,
    },
}

fn parse_resource_type(s: &str) -> Result<ResourceType, Box<dyn Error + Send + Sync + 'static>> {
    match s {
        "req" | "request" => Ok(ResourceType::Request),
        "resp" | "response" => Ok(ResourceType::Response),
        "script" => Ok(ResourceType::Script),
        _ => panic!("unrecognized resource type {}", s),
    }
}

fn parse_header(
    s: &str,
) -> Result<(HeaderName, HeaderValue), Box<dyn Error + Send + Sync + 'static>> {
    let idx = s
        .find(':')
        .ok_or_else(|| format!("could not find ':' in {}", s))?;
    Ok((s[0..idx].parse()?, s[idx + 1..].parse()?))
}

pub(crate) fn parse_http_version(
    s: &str,
) -> Result<crate::http::Version, Box<dyn Error + Send + Sync + 'static>> {
    match s {
        "0.9" => Ok(crate::http::Version::HTTP_09),
        "1.0" => Ok(crate::http::Version::HTTP_10),
        "1.1" => Ok(crate::http::Version::HTTP_11),
        "2.0" => Ok(crate::http::Version::HTTP_2),
        "3.0" => Ok(crate::http::Version::HTTP_3),
        _ => panic!("unrecognized http version: {}", s),
    }
}

pub(crate) fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.commands {
        SubCommand::Send { options } => http::send(options)?,
        SubCommand::List { options } => list(options),
        SubCommand::Retry { options } => retry(options),
        SubCommand::Describe { options } => describe(options),
    };
    Ok(())
}

fn list(options: ListOptions) {
    match options.resource_type {
        ResourceType::Request => {
            let table = db::retrive_resources::<crate::model::RequestTable>(options.limit);
            if table.len() > 0 {
                crate::display::display(&table);
            } else {
                println!("You have not sent any request yet.");
            }
        }
        ResourceType::Response => {
            let table = db::retrive_resources::<crate::model::ResponseTable>(options.limit);
            if table.len() > 0 {
                crate::display::display(&table);
            } else {
                println!("You have not got any response yet.");
            }
        }
        ResourceType::Script => todo!(),
    }
}

fn describe(options: DescribeOptions) {
    println!(
        "resource type: {:?}, id: {}",
        options.resource_type, options.request_id
    );
}
