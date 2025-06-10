use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    println!("{version}");
    Ok(())
}
