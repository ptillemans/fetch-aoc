use anyhow::Result;
use fetch_aoc::{Config, fetch_input, fetch_challenge, generate_main_rs, generate_cargo_toml, add_member_to_workspace, generate_lib_rs, };

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new();
    println!("year: {}", cfg.year);
    println!("day: {}", cfg.day);
    fetch_challenge(&cfg).await?;
    fetch_input(&cfg).await?; 
    generate_main_rs(&cfg)?;
    generate_lib_rs(&cfg)?;
    generate_cargo_toml(&cfg)?;
    add_member_to_workspace(&cfg)?;
    println!("Good luck");
    Ok(())
}
