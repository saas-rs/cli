pub(super) mod identity_provider;
pub(super) mod storage_provider;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::enum_variant_names)]
pub enum Subcommand {
    #[command(name = "identity-provider", alias = "identityProvider")]
    IdentityProvider(identity_provider::Opts),

    #[command(name = "storage-provider", alias = "storageProvider")]
    StorageProvider(storage_provider::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::IdentityProvider(identity_provider::Opts { provider }) => {
            identity_provider::run(provider).await?;
        }
        Subcommand::StorageProvider(storage_provider::Opts { provider }) => {
            storage_provider::run(provider).await?;
        }
    }
    Ok(())
}
