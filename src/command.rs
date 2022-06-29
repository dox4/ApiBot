use std::error::Error;

use clap::{Parser, Subcommand};
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

use crate::http::{self, Method};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) commands: SubCommand,
}
#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    Send {
        #[clap(value_parser, help = "url to send request to.")]
        url: String,
        #[clap(
            short = 'X',
            long,
            value_parser,
            default_value = "GET",
            help = "HTTP request method, default is 'GET'."
        )]
        method: Method,
        #[clap(short = 'H',long, value_parser = parse_header, multiple = true, help = "headers of the HTTP request.")]
        headers: Vec<(HeaderName, HeaderValue)>,
        #[clap(long, default_value = "1.1", help = "http version, default is 1.1.")]
        http: f64,
        #[clap(short, long, help = "data to send with the request.")]
        data: Option<String>,
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

pub(crate) async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.commands {
        // SubCommand::Get { url } => {
        //     let req = client.get(url);
        //     let req = req.build()?;
        //     let resp = client.execute(req).await?;
        //     if let Some(ct) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
        //         if ct.to_str().unwrap().contains("application/json") {
        //             let obj = resp.json::<HashMap<String, Value>>().await?;
        //             println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        //         } else {
        //             println!("{}", resp.text().await?);
        //         }
        //     }
        // }
        // SubCommand::Post { url: _ } => todo!(),
        SubCommand::List {} => todo!(),
        SubCommand::Retry {} => todo!(),
        SubCommand::Send {
            url,
            method,
            headers,
            http,
            data,
        } => http::send(url, method, headers, http, data).await?,
        SubCommand::Describe {} => todo!(),
    };
    Ok(())
}
