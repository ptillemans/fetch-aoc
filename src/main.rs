use clap::Parser;

#[derive(Parser)]
#[command(author, version, long_about = None)]
struct Cli {
    year: u16,
    day: u16,
}

fn main() {
    println!("Hello, world!");
}
 