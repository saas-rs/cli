use std::error::Error;
use tonic::transport::{Channel, ClientTlsConfig};

pub async fn new_channel(lazy: bool, api_url: String) -> Result<Channel, Box<dyn Error>> {
    let mut endpoint = Channel::from_shared(api_url.clone())?;

    // Configure TLS
    if api_url.starts_with("https") {
        let tls_config = ClientTlsConfig::new().with_enabled_roots().assume_http2(true);
        endpoint = endpoint.tls_config(tls_config)?;
    }

    // Connect
    let channel = match lazy {
        true => endpoint.connect_lazy(),
        false => {
            let channel = endpoint.connect().await?;
            log::info!("Connected to {api_url}");
            channel
        }
    };

    Ok(channel)
}
