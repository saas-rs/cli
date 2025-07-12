use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::generate_request::{self, Field, Model};
use crate::protocol::saas_rs::user::v1::GenerateRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Which service this new model is for
    #[arg(long = "service")]
    pub service: String,

    /// Version of the service that this model is for
    #[arg(long = "version")]
    pub version: u32,

    /// Name of the new model
    #[arg(value_name = "name")]
    pub name: String,

    /// Fields to add to the new model
    #[arg(value_name = "fields")]
    pub fields: Vec<String>,
}

pub async fn run(
    service: String,
    version: u32,
    name: String,
    fields: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight(false).await?;
    let req = {
        let fields = fields
            .iter()
            .map(|name| {
                let (name, r#type) = {
                    let parts: Vec<String> = name.split(':').map(str::to_string).collect();
                    if parts.len() == 2 {
                        (parts[0].clone(), Some(parts[1].clone()))
                    } else {
                        (name.clone(), None)
                    }
                };
                Field { name, r#type }
            })
            .collect();
        GenerateRequest {
            project_id,
            snapshot: Some(snapshot),
            what: Some(generate_request::What::Model(Model {
                service,
                version,
                name,
                fields,
            })),
        }
    };
    do_generate(req).await
}
