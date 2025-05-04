use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::generate_request::{Init, What};
use crate::protocol::saas_rs::user::v1::GenerateRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Your brand. This becomes your proto package as well as your CLI binary name; eg "heroku", "stripe"
    #[arg(long = "brand")]
    pub brand: String,
}

pub async fn run(brand: String) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight().await?;
    let req = GenerateRequest {
        project_id,
        snapshot: Some(snapshot),
        what: Some(What::Init(Init { brand })),
    };
    do_generate(req).await
}
