use clap::Parser;

pub(super) mod issue;
pub(super) mod project;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::enum_variant_names)]
pub enum Subcommand {
    #[command(name = "issue")]
    Issue(issue::Opts),

    #[command(name = "project")]
    Project(project::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Issue(issue::Opts {
            title,
            project_id,
            description,
        }) => {
            issue::run(title, project_id, description).await?;
        }
        Subcommand::Project(project::Opts { id, name }) => {
            project::run(id, name).await?;
        }
    }
    Ok(())
}
