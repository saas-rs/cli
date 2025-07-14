use crate::cmd::generate::{do_generate, do_generate_preflight};
use crate::protocol::saas_rs::user::v1::{
    generate_request::{
        self,
        use_storage_adapter::{Provider, Type},
        UseStorageAdapter,
    },
    GenerateRequest,
};
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Opts {
    /// The storage provider (eg Memory, MongoDB, Redis)
    #[arg(long = "provider", value_enum)]
    pub provider: Provider,

    /// The storage type (one of Config, Object, Session)
    #[arg(long = "type", value_enum)]
    pub r#type: Type,
}

pub async fn run(provider: Provider, r#type: Type) -> Result<(), Box<dyn std::error::Error>> {
    let (project_id, snapshot) = do_generate_preflight(false).await?;
    let req = {
        GenerateRequest {
            project_id,
            snapshot: Some(snapshot),
            what: Some(generate_request::What::UseStorageAdpater(UseStorageAdapter {
                provider: provider as i32,
                r#type: r#type as i32,
            })),
        }
    };
    do_generate(req).await
}

impl ValueEnum for Provider {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Memory, Self::MongoDb, Self::Redis]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.as_str_name()))
    }
}

impl ValueEnum for Type {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Config, Self::Object, Self::Session]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.as_str_name()))
    }
}
