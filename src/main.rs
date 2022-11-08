use std::path::PathBuf;
use std::sync::Arc;
use std::fs;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use reqwest::{cookie::Jar, Url};

#[derive(Parser)]
#[command(author, version, long_about = None)]
struct Cli {
    #[arg(short, long, env("AOC_YEAR"))]
    year: Option<u16>,
    #[arg(short, long, env("AOC_DAY"))]
    day: Option<u16>,
    #[arg(short, long, env("AOC_SESSION"))]
    session: String,
}

struct Config {
    year: u16,
    day: u16,
    session: String,
    data_folder: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        let now = chrono::Utc::now();
        let cli = Cli::parse();
        let year = cli.year.unwrap_or(now.year() as u16);
        let day = cli.day.unwrap_or(now.day() as u16);
        let session = cli.session;
        let data_folder = PathBuf::from("data");
        Config { year, day, session, data_folder }
    }
}

async fn fetch_page(cfg: &Config, page: &str) -> Result<String> {
    let aoc_url = "https://adventofcode.com/".parse::<Url>().unwrap();
    let cookie = format! {"session={}", cfg.session};

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &aoc_url);
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .build()?;

    let url = aoc_url.join(page)?;
    println!("fetching {:?}", url);
    client
        .get(url)
        .send()
        .await?
        .text()
        .await
        .context("getting page from aoc site")
}

fn create_data_folder(cfg: &Config) -> Result<()> {
    fs::create_dir_all(&cfg.data_folder.as_path()).context("Create data folder")
}

async fn fetch_challenge(cfg: &Config) -> Result<String> {
    let path = format!("{}/day/{}", cfg.year, cfg.day);
    fetch_page(cfg, &path).await
}

async fn fetch_input(cfg: &Config) -> Result<()> {
    let path = format!("{}/day/{}/input", cfg.year, cfg.day);
    let input = fetch_page(cfg, &path).await?;
    let filename = cfg.data_folder.join("input.txt");
    fs::write(&filename, &input)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new();
    println!("Hello, world!");
    println!("year: {}", cfg.year);
    println!("day: {}", cfg.day);
    println!("session: {}", cfg.session);
    create_data_folder(&cfg)?;
    let challenge = fetch_challenge(&cfg).await?;
    fetch_input(&cfg).await?; 
    let markdown = html2md::parse_html(&challenge);
    println!("page: {}", markdown);

    Ok(())
}
