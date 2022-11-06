use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use reqwest::{cookie::Jar, Url};

#[derive(Parser)]
#[command(author, version, long_about = None)]
struct Cli {
    year: Option<u16>,
    day: Option<u16>,
    #[arg(short, long, env("AOC_SESSION"))]
    session: String,
}

struct Config {
    year: u16,
    day: u16,
    session: String,
}

impl Config {
    pub fn new() -> Config {
        let now = chrono::Utc::now();
        let cli = Cli::parse();
        let year = cli.year.unwrap_or(now.year() as u16);
        let day = cli.year.unwrap_or(now.day() as u16);
        let session = cli.session;
        Config { year, day, session }
    }
}

async fn fetch_page(cfg: Config, page: &str) -> Result<String> {
    let aoc_url = "https://adventofcode.com/".parse::<Url>().unwrap();
    let cookie = format! {"session={}", cfg.session};

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &aoc_url);
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .build()?;

    let url = aoc_url.join(page)?;
    client
        .get(url)
        .send()
        .await?
        .text()
        .await
        .context("getting page from aoc site")
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new();
    println!("Hello, world!");
    println!("year: {}", cfg.year);
    println!("day: {}", cfg.day);
    println!("session: {}", cfg.session);
    let page: String = fetch_page(cfg, "2018/day/1").await?;
    let markdown = html2md::parse_html(&page);
    println!("page: {}", markdown);

    Ok(())
}
