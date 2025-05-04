use crate::apiclient;
use crate::protocol::saas_rs::user::v1::DeleteServiceInstanceRequest;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Service Instance ID
    #[arg(value_name = "ID")]
    pub(super) id: String,
}

pub async fn run(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let req = DeleteServiceInstanceRequest { id };
    let _res = client.delete_service_instance(req).await?.into_inner();
    eprintln!("Deleted");
    Ok(())
}
