pub(super) mod controller;
mod do_generate;
mod do_generate_preflight;
pub(super) mod feature;
pub(super) mod model;
pub(super) mod resource;
pub(super) mod service;
pub(super) mod uuid_v4;
pub(super) mod xid;

use clap::Parser;
pub use do_generate::do_generate;
pub use do_generate_preflight::do_generate_preflight;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::enum_variant_names)]
pub enum Subcommand {
    #[command(name = "controller")]
    Controller(controller::Opts),

    #[command(name = "feature")]
    Feature(feature::Opts),

    #[command(name = "model")]
    Model(model::Opts),

    #[command(name = "resource")]
    Resource(resource::Opts),

    #[command(name = "service")]
    Service(service::Opts),

    #[command(name = "xid")]
    Xid(xid::Opts),

    #[command(name = "uuidv4")]
    UuidV4(xid::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Controller(controller::Opts {
            service,
            version,
            resource,
        }) => {
            controller::run(service, version, resource).await?;
        }
        Subcommand::Feature(feature::Opts { service, version }) => {
            feature::run(service, version).await?;
        }
        Subcommand::Model(model::Opts {
            service,
            version,
            name,
            fields,
        }) => {
            model::run(service, version, name, fields).await?;
        }
        Subcommand::Resource(resource::Opts {
            service,
            version,
            name,
            fields,
        }) => {
            resource::run(service, version, name, fields).await?;
        }
        Subcommand::Service(service::Opts {
            name,
            resources,
            version,
            with_cli,
        }) => {
            service::run(name, resources, version, with_cli).await?;
        }
        Subcommand::Xid(xid::Opts { n }) => {
            xid::run(n).await?;
        }
        Subcommand::UuidV4(xid::Opts { n }) => {
            uuid_v4::run(n).await?;
        }
    }
    Ok(())
}
