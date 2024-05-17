use std::{
    io::{stdin, stdout, Write},
    process,
};

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

    println!("Установка пакета {}", args.install);
    let url = format!("https://aur.archlinux.org/rpc/v5/search/{}", args.install);

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let response = response.json::<Response>().await.unwrap();
    let items = response.results;
    let items: Vec<&Item> = items
        .iter()
        .filter(|item| args.install == item.package_base)
        .collect();
    let item = items.get(0);
    if item.is_none() {
        panic!("Не найдено ни одного пакета для установки");
    }

    if let Some(item) = item {
        println!("{:#?}", item);

        let mut s = String::new();
        print!("Хотите ли вы установить {}: ", args.install);
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("неправильный string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        if s == "да" {
            let url = format!("{}{}", item.url, item.urlpath);
            println!("Installing {} from {}", args.install, url);
        }
    }

    Ok(())
}
