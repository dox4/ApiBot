use std::error::Error;

use clap::{Parser, Subcommand};
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

use crate::db;
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

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    Send {
        #[clap(flatten)]
        options: SendOptions,
    },
    List {},
    Retry {},
    Describe {},
}

fn parse_header(
    s: &str,
) -> Result<(HeaderName, HeaderValue), Box<dyn Error + Send + Sync + 'static>> {
    let idx = s
        .find(':')
        .ok_or_else(|| format!("could not find ':' in {}", s))?;
    Ok((s[0..idx].parse()?, s[idx + 1..].parse()?))
}

fn parse_http_version(
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
        SubCommand::List {} => todo!(),
        SubCommand::Retry {} => todo!(),
        SubCommand::Send { options } => http::send(options)?,
        SubCommand::Describe {} => todo!(),
    };
    Ok(())
}
