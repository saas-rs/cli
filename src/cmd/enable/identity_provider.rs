use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::{
    GenerateRequest,
    generate_request::{self, UseIdentityProvider, use_identity_provider::Provider},
};
use clap::{
    builder::PossibleValue,
    {Parser, ValueEnum},
};

#[derive(Debug, Parser)]
pub struct Opts {
    /// The identity provider
    #[arg(value_name = "provider", value_enum)]
    pub provider: Provider,
}

pub async fn run(provider: Provider) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight(false).await?;
    let req = {
        GenerateRequest {
            project_id,
            snapshot: Some(snapshot),
            what: Some(generate_request::What::UseIdentityProvider(UseIdentityProvider {
                provider: provider as i32,
            })),
        }
    };
    do_generate(req).await
}

impl ValueEnum for Provider {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Amazon,
            Self::Apple,
            Self::DigitalOcean,
            Self::Facebook,
            Self::GitHub,
            Self::GitLab,
            Self::Google,
            Self::Instagram,
            Self::Linode,
            Self::Microsoft,
            Self::Okta,
            Self::Twitter,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.as_str_name()))
    }
}
