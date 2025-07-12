use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::generate_request::{self, Controller};
use crate::protocol::saas_rs::user::v1::GenerateRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Service that this controller is for
    #[arg(long = "service")]
    pub service: String,

    /// Version of the service that this controller is for
    #[arg(long = "version")]
    pub version: u32,

    /// Resource that this controller is for
    #[arg(value_name = "resource")]
    pub resource: String,
}

pub async fn run(service: String, version: u32, resource: String) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight(false).await?;
    let req = {
        GenerateRequest {
            project_id,
            snapshot: Some(snapshot),
            what: Some(generate_request::What::Controller(Controller {
                resource,
                service,
                version,
            })),
        }
    };
    do_generate(req).await
}
