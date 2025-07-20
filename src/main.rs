mod apiclient;
mod cmd;
mod config;
mod consts;
mod kind;
pub(crate) mod protocol;
pub(crate) mod util;

use clap::Parser;

pub static OUTPUTS: &[&str] = &["arrow", "json", "markdown", "ps", "wide"];

//-----------------------------------------------------------------------------
// Commands
//-----------------------------------------------------------------------------
#[derive(Debug, Parser)]
pub struct AppOpts {
    /// API Key used to authorize requests
    #[arg(long = "api-key")]
    pub api_key: Option<String>,

    /// Customize the API URL
    #[arg(long = "api-url")]
    pub api_url: Option<String>,

    #[command(subcommand)]
    pub subcommand: cmd::Subcommand,
}

//-----------------------------------------------------------------------------
// main
//-----------------------------------------------------------------------------
#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let _ = tokio_rustls::rustls::crypto::ring::default_provider().install_default();

    let opts = AppOpts::parse();
    match opts.subcommand {
        cmd::Subcommand::Create(cmd::create::Opts { subcommand }) => {
            cmd::create::run(subcommand).await?;
        }
        cmd::Subcommand::Delete(cmd::delete::Opts { subcommand }) => {
            cmd::delete::run(subcommand).await?;
        }
        cmd::Subcommand::Enable(cmd::enable::Opts { subcommand }) => {
            cmd::enable::run(subcommand).await?;
        }
        cmd::Subcommand::Generate(cmd::generate::Opts { subcommand }) => {
            cmd::generate::run(subcommand).await?;
        }
        cmd::Subcommand::Get(cmd::get::Opts { subcommand }) => {
            cmd::get::run(subcommand).await?;
        }
        cmd::Subcommand::History(cmd::history::Opts { offset, limit, output }) => {
            cmd::history::run(offset, limit, output).await?;
        }
        cmd::Subcommand::Initialize(cmd::initialize::Opts { brand, path }) => {
            cmd::initialize::run(brand, path).await?;
        }
        cmd::Subcommand::List(cmd::list::Opts { subcommand }) => {
            cmd::list::run(subcommand).await?;
        }
        cmd::Subcommand::Login(cmd::login::Opts {
            api_url,
            console_url,
            browser,
        }) => {
            cmd::login::run(api_url, console_url, browser).await?;
        }
        cmd::Subcommand::Logout(cmd::logout::Opts {}) => {
            cmd::logout::run().await?;
        }
        cmd::Subcommand::Version(cmd::version::Opts {}) => {
            cmd::version::run().await?;
        }
    }
    Ok(())
}
