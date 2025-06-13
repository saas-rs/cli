use clap::Parser;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
pub struct Opts {}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(version) = VERSION {
        println!("{version}");
    } else {
        eprintln!("(Unknown)");
    }
    Ok(())
}
