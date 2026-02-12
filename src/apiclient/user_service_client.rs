use super::channel::new_channel;
use crate::AppOpts;
use crate::protocol::saas_rs::user::v1::user_client::UserClient;
use crate::{config, consts};
use clap::Parser;
use std::error::Error;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::{Request, Status, metadata::MetadataValue};

pub async fn new_user_service_client()
-> Result<UserClient<InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>>, Box<dyn Error>> {
    new_user_service_client_with_ignore_config(false).await
}

pub async fn new_user_service_client_with_ignore_config(
    ignore_config: bool,
) -> Result<UserClient<InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>>, Box<dyn Error>> {
    // Connect
    let api_url = get_api_url(ignore_config)?;
    let channel = new_channel(false, api_url.clone()).await?;

    // Construct client and register authorization interceptor
    let client = UserClient::with_interceptor(channel, intercept as fn(Request<()>) -> Result<Request<()>, Status>);

    Ok(client)
}

pub fn get_api_key() -> Result<Option<String>, Box<dyn Error>> {
    let opts = AppOpts::parse();
    let api_key = match opts.api_key {
        Some(api_key) => Some(api_key),
        None => match std::env::var(consts::env_vars::SAAS_RS_API_KEY) {
            Ok(api_key) => Some(api_key),
            _ => config::load()?.api_key,
        },
    };
    Ok(api_key)
}

pub fn get_api_url(ignore_config: bool) -> Result<String, Box<dyn Error>> {
    let app_opts = AppOpts::parse();
    let api_url = match app_opts.api_url {
        Some(api_url) => api_url,
        None => {
            if ignore_config {
                consts::env_vars::API_URL.to_string()
            } else {
                let cfg = config::load()?;
                match cfg.api_url {
                    Some(api_url) => api_url,
                    None => consts::env_vars::API_URL.to_string(),
                }
            }
        }
    };
    Ok(api_url)
}

fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
    let api_key = match get_api_key() {
        Err(e) => return Err(Status::internal(e.to_string())),
        Ok(None) => return Err(Status::unauthenticated("Login required")),
        Ok(Some(api_key)) => api_key,
    };
    let authorization = format!("Bearer {api_key}");
    let auth_meta_value: MetadataValue<_> = match authorization.parse() {
        Ok(auth_meta_value) => auth_meta_value,
        Err(e) => return Err(Status::invalid_argument(e.to_string())),
    };
    req.metadata_mut().insert("authorization", auth_meta_value);
    Ok(req)
}
