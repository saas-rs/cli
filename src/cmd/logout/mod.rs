use crate::config;
use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
pub struct Opts {}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut cfg = config::load()?;
    if cfg.api_key.is_none() {
        eprintln!("You are already logged out");
    } else {
        cfg.api_key = None;
        cfg.api_url = None;
        let config_filename = config::save(&cfg)?;
        eprintln!("API Key removed from {config_filename}");
    }

    Ok(())
}
