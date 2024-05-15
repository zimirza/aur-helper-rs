use std::{env::VarError, process};

use clap::Parser;
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.count != 1 {
        process::exit(1);
    }

    println!("Installing package {}", args.install);
    let url = format!("https://aur.archlinux.org/rpc/v5/search/{}", args.install);

    let client = reqwest::Client::new();
    // let res = client.get(url).send().await?;
    // let json = res.json().await.map_err(|e| {
    //     eprintln!("Cannot get {e}");
    //     VarError::NotPresent
    // })?;
    // println!("{:#?}", res.json().await?);

    Ok(())
}
