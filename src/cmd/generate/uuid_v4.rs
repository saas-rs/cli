use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Number of ids to generate
    #[arg(short = 'n', default_value = "1")]
    pub n: u16,
}

pub async fn run(n: u16) -> Result<(), Box<dyn std::error::Error>> {
    for _i in 1..=n {
        let value = Uuid::new_v4();
        eprintln!("{}", value);
    }
    Ok(())
}
