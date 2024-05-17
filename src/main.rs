use std::process;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Aur {
    response: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    install: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub resultcount: i64,
    pub results: Vec<Item>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: i64,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "LastModified")]
    pub last_modified: i64,
    #[serde(rename = "Maintainer")]
    pub maintainer: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NumVotes")]
    pub num_votes: i64,
    #[serde(rename = "OutOfDate")]
    pub out_of_date: Value,
    #[serde(rename = "PackageBase")]
    pub package_base: String,
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: i64,
    #[serde(rename = "Popularity")]
    pub popularity: f64,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "URLPath")]
    pub urlpath: String,
    #[serde(rename = "Version")]
    pub version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.count != 1 {
        process::exit(1);
    }

    println!("Installing package {}", args.install);
    let url = format!("https://aur.archlinux.org/rpc/v5/search/{}", args.install);

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let response = response.json::<Response>().await.unwrap();
    println!("{:#?}", response);

    Ok(())
}
