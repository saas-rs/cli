use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::generate_request::{self, Feature};
use crate::protocol::saas_rs::user::v1::GenerateRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Which service this feature is for
    #[arg(long = "service")]
    pub service: Option<String>,

    /// Version of the service that this feature is for
    #[arg(long = "version")]
    pub version: Option<u32>,
}

pub async fn run(service: Option<String>, version: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight().await?;
    let req = GenerateRequest {
        project_id,
        snapshot: Some(snapshot),
        what: Some(generate_request::What::Feature(Feature { service, version })),
    };
    do_generate(req).await
}
