use crate::apiclient;
use crate::protocol::saas_rs::user::v1::{CreateIssueRequest, Issue};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Title
    #[arg(long = "title")]
    pub(super) title: String,

    /// Project ID
    #[arg(long = "project-id")]
    pub(super) project_id: Option<String>,

    /// Description
    #[arg(long = "description")]
    pub(super) description: Option<String>,
}

pub async fn run(
    title: String,
    project_id: Option<String>,
    description: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let issue = Issue {
        title,
        project_id,
        description,
        ..Default::default()
    };
    let req = CreateIssueRequest { issue: Some(issue) };
    let _res = client.create_issue(req).await?.into_inner();
    eprintln!("Created");
    Ok(())
}
