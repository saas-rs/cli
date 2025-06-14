use crate::apiclient;
use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::generate_request::{self, Feature};
use crate::protocol::saas_rs::user::v1::{FindManyGenerationFeaturesRequest, GenerateRequest, GenerationFeatureFilter};
use clap::Parser;
use pbjson_types::FieldMask;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Which feature ID
    #[arg(long = "id", group = "id_or_name")]
    pub id: Option<String>,

    /// Which feature Name
    #[arg(long = "name", group = "id_or_name")]
    pub name: Option<String>,

    /// Which service this feature is for
    #[arg(long = "service")]
    pub service: Option<String>,

    /// Version of the service that this feature is for
    #[arg(long = "version")]
    pub version: Option<u32>,
}

pub async fn run(
    id: Option<String>,
    name: Option<String>,
    service: Option<String>,
    version: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight().await?;
    let id = match id {
        Some(id) => id,
        None => {
            let mut client = apiclient::new_user_service_client().await?;
            let req = FindManyGenerationFeaturesRequest {
                filter: Some(GenerationFeatureFilter {
                    name,
                    ..Default::default()
                }),
                field_mask: Some(FieldMask {
                    paths: vec!["id".to_string()],
                }),
                ..Default::default()
            };
            let res = client.find_many_generation_features(req).await?.into_inner();
            match res.generation_features.first() {
                Some(generation_feature) => generation_feature.id.clone(),
                None => return Err("No such feature".into()),
            }
        }
    };
    let req = GenerateRequest {
        project_id,
        snapshot: Some(snapshot),
        what: Some(generate_request::What::Feature(Feature { id, service, version })),
    };
    do_generate(req).await
}
