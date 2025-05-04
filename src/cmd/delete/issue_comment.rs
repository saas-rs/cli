use crate::apiclient;
use crate::protocol::saas_rs::user::v1::DeleteIssueCommentRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// IssueComment Comment ID
    #[arg(value_name = "ID")]
    pub(super) id: String,
}

pub async fn run(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let req = DeleteIssueCommentRequest { id };
    let _res = client.delete_issue_comment(req).await?.into_inner();
    eprintln!("Deleted");
    Ok(())
}
