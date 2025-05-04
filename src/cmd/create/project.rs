use crate::apiclient;
use crate::protocol::saas_rs::user::v1::{CreateProjectRequest, Project};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Project ID
    #[arg(long = "id")]
    pub(super) id: Option<String>,

    /// Project Name
    #[arg(value_name = "NAME")]
    pub(super) name: String,
}

pub async fn run(id: Option<String>, name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let project = Project {
        id: id.unwrap_or_default(),
        name,
        ..Default::default()
    };
    let req = CreateProjectRequest { project: Some(project) };
    let _res = client.create_project(req).await?.into_inner();
    eprintln!("Created");
    Ok(())
}
