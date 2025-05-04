use clap::Parser;

mod issue;
pub(super) mod issue_comment;
pub(super) mod linked_account;
pub(super) mod plan;
pub(super) mod project;
pub(super) mod service;
pub(super) mod service_instance;

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

    #[command(name = "issue-comment", alias = "issueComment")]
    IssueComment(issue_comment::Opts),

    #[command(name = "linked-account", alias = "linkedAccount")]
    LinkedAccount(linked_account::Opts),

    #[command(name = "plan")]
    Plan(plan::Opts),

    #[command(name = "project")]
    Project(project::Opts),

    #[command(name = "service")]
    Service(service::Opts),

    #[command(name = "service-instance", alias = "serviceInstance")]
    ServiceInstance(service_instance::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Issue(issue::Opts { id }) => {
            issue::run(id).await?;
        }
        Subcommand::IssueComment(issue_comment::Opts { id }) => {
            issue_comment::run(id).await?;
        }
        Subcommand::LinkedAccount(linked_account::Opts { id }) => {
            linked_account::run(id).await?;
        }
        Subcommand::Plan(plan::Opts { id }) => {
            plan::run(id).await?;
        }
        Subcommand::Project(project::Opts { id }) => {
            project::run(id).await?;
        }
        Subcommand::Service(service::Opts { id }) => {
            service::run(id).await?;
        }
        Subcommand::ServiceInstance(service_instance::Opts { id }) => {
            service_instance::run(id).await?;
        }
    }
    Ok(())
}
