use crate::apiclient;
use crate::protocol::saas_rs::user::v1::DeletePlanRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Plan ID
    #[arg(value_name = "ID")]
    pub(super) id: String,
}

pub async fn run(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let req = DeletePlanRequest { id };
    let _res = client.delete_plan(req).await?.into_inner();
    eprintln!("Deleted");
    Ok(())
}
