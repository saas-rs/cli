use crate::apiclient;
use crate::protocol::saas_rs::user::v1::DeleteLinkedAccountRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Linked Account ID
    #[arg(value_name = "ID")]
    pub(super) id: String,
}

pub async fn run(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let req = DeleteLinkedAccountRequest { id };
    let _res = client.delete_linked_account(req).await?.into_inner();
    eprintln!("Deleted");
    Ok(())
}
