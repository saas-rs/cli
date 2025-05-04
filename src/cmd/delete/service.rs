use crate::apiclient;
use crate::protocol::saas_rs::user::v1::DeleteServiceRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Service ID
    #[arg(value_name = "ID")]
    pub(super) id: String,
}

pub async fn run(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let req = DeleteServiceRequest { id };
    let _res = client.delete_service(req).await?.into_inner();
    eprintln!("Deleted");
    Ok(())
}
