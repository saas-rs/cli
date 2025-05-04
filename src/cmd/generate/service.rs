use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::generate_request::{self, Service};
use crate::protocol::saas_rs::user::v1::GenerateRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Name of the new service (eg user, admin, lambda)
    #[arg(value_name = "name")]
    pub name: String,

    /// Resources to manage
    #[arg(value_name = "models", default_values_t = ["account".to_string(), "linked-account".to_string()])]
    pub resources: Vec<String>,

    /// The protocol version
    #[arg(long = "version", default_value_t = 1)]
    pub version: u32,

    /// Whether to generate an associated CLI
    #[arg(long = "with-cli", default_value_t = false)]
    pub with_cli: bool,
}

pub async fn run(
    name: String,
    resources: Vec<String>,
    version: u32,
    with_cli: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight().await?;
    let req = {
        GenerateRequest {
            project_id,
            snapshot: Some(snapshot),
            what: Some(generate_request::What::Service(Service {
                name,
                resources,
                version,
                with_cli,
            })),
        }
    };
    do_generate(req).await
}
