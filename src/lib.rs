use std::{path::PathBuf, collections::BTreeMap};
use std::sync::Arc;
use std::fs;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use reqwest::{cookie::Jar, Url};
use handlebars::Handlebars;
use toml_edit::Document;

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

pub struct Config {
    pub year: u16,
    pub day: u16,
    session: String,
}

impl Config {
    pub fn new() -> Config {
        let now = chrono::Utc::now();
        let cli = Cli::parse();
        let year = cli.year.unwrap_or(now.year() as u16);
        let day = cli.day.unwrap_or(now.day() as u16);
        let session = cli.session;
        Config { year, day, session }
    }
}

pub async fn fetch_page(cfg: &Config, page: &str) -> Result<String> {
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
fn project_name(cfg: &Config) -> String {
    format!("aoc-{}-{}", cfg.year, cfg.day)
}

pub fn write_file(cfg: &Config, name: &PathBuf, contents: &str) -> Result<()> {
    let base = PathBuf::from(project_name(&cfg));
    let fullname = base.join(name);
    fs::create_dir_all(fullname.parent().unwrap())?;
    if !fullname.exists() {
        fs::write(&fullname, contents).context(format!("Writing {:?}", &fullname))
    } else {
        println!("File {:?} already exists.", &fullname);
        Ok(())
    }
}

pub async fn fetch_challenge(cfg: &Config) -> Result<()> {
    let path = format!("{}/day/{}", cfg.year, cfg.day);
    let challenge = fetch_page(cfg, &path).await?;
    let markdown = html2md::parse_html(&challenge);
    let data_folder = PathBuf::from("data");
    fs::create_dir_all(data_folder.as_path()).context("Create data folder")?;
    let filename = data_folder.join("challenge.md");
    write_file(&cfg, &filename,  &markdown)?;
    Ok(())
}

pub async fn fetch_input(cfg: &Config) -> Result<()> {
    let path = format!("{}/day/{}/input", cfg.year, cfg.day);
    let input = fetch_page(cfg, &path).await?;
    let filename = PathBuf::from("data/input.txt");
    write_file(&cfg, &filename, &input)?;
    Ok(())
}

const MAIN_TEMPLATE: &str = include_str!("../template/main.rs");
const LIB_TEMPLATE: &str = include_str!("../template/lib.rs");
const CARGO_TEMPLATE: &str = include_str!("../template/Cargo.toml");

fn create_template_engine(_cfg: &Config) -> Result<Handlebars> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("main_rs", MAIN_TEMPLATE)?;
    handlebars.register_template_string("lib_rs", LIB_TEMPLATE)?;
    handlebars.register_template_string("cargo_toml", CARGO_TEMPLATE)?;
    Ok(handlebars)
}

fn create_template_data(cfg: &Config) -> BTreeMap<String, String> {
    let mut data: BTreeMap<String, String> = BTreeMap::new();
    data.insert("year".to_string(), cfg.year.to_string());
    data.insert("day".to_string(), cfg.day.to_string());
    data
}

pub fn generate_main_rs(cfg: &Config) -> Result<()> {
    let src_folder = PathBuf::from("src");
    fs::create_dir_all(src_folder.as_path()).context("Create src folder")?;
    let filename = src_folder.join("main.rs");
    let engine = create_template_engine(cfg)?;
    let data = create_template_data(cfg);
    let main_rs = engine.render("main_rs", &data).context("Rendering source code")?;
    write_file(&cfg, &filename, &main_rs).context("Writing source code")
}

pub fn generate_lib_rs(cfg: &Config) -> Result<()> {
    let src_folder = PathBuf::from("src");
    fs::create_dir_all(src_folder.as_path()).context("Create src folder")?;
    let filename = src_folder.join("lib.rs");
    let engine = create_template_engine(cfg)?;
    let data = create_template_data(cfg);
    let main_rs = engine.render("lib_rs", &data).context("Rendering source code")?;
    write_file(&cfg, &filename, &main_rs).context("Writing source code")
}

pub fn generate_cargo_toml(cfg: &Config) -> Result<()> {
    let filename = PathBuf::from("Cargo.toml");
    let engine = create_template_engine(cfg)?;
    let data = create_template_data(cfg);
    let main_rs = engine.render("cargo_toml", &data).context("Rendering Cargo.toml")?;
    write_file(&cfg, &filename, &main_rs).context("Writing Cargo.toml")
}

pub fn add_member_to_workspace(cfg: &Config) -> Result<()> {
    let cargo_contents = fs::read_to_string("Cargo.toml")?;
    let mut doc = cargo_contents.parse::<Document>().context("Parsing Cargo.toml")?;
    let members = doc["workspace"]["members"].as_array_mut().expect("updating members in workspace");
    if !members.iter().any(|v| v.to_string() == project_name(&cfg).to_string()) {
        println!("Adding {} to workspace", project_name(&cfg));
        members.push(project_name(&cfg));
    }
    fs::rename("Cargo.toml", "Cargo.bak")?;
    fs::write("Cargo.toml", doc.to_string())?;
    Ok(())
}
