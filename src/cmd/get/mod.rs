use clap::Parser;

pub(super) mod account;
pub(super) mod api_key;
pub(super) mod generator;
pub(super) mod issue;
pub(super) mod issue_comment;
pub(super) mod linked_account;
pub(super) mod output;
pub(super) mod plan;
pub(super) mod project;
pub(super) mod service;
pub(super) mod service_instance;
pub(super) mod ssh_key;

pub use output::output;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::enum_variant_names)]
pub enum Subcommand {
    #[command(name = "account")]
    Account(account::Opts),

    #[command(name = "api-key", alias = "apiKey")]
    ApiKey(api_key::Opts),

    #[command(name = "generator")]
    Generator(generator::Opts),

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

    #[command(name = "ssh-key", alias = "sshKey")]
    SshKey(ssh_key::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Account(account::Opts { id, output }) => {
            account::run(id, output).await?;
        }
        Subcommand::ApiKey(api_key::Opts { id, output }) => {
            api_key::run(id, output).await?;
        }
        Subcommand::Generator(generator::Opts { id, output }) => {
            generator::run(id, output).await?;
        }
        Subcommand::Issue(issue::Opts { id, output }) => {
            issue::run(id, output).await?;
        }
        Subcommand::IssueComment(issue_comment::Opts { id, output }) => {
            issue_comment::run(id, output).await?;
        }
        Subcommand::LinkedAccount(linked_account::Opts { id, output }) => {
            linked_account::run(id, output).await?;
        }
        Subcommand::Plan(plan::Opts { id, output }) => {
            plan::run(id, output).await?;
        }
        Subcommand::Project(project::Opts { id, output }) => {
            project::run(id, output).await?;
        }
        Subcommand::Service(service::Opts { id, output }) => {
            service::run(id, output).await?;
        }
        Subcommand::ServiceInstance(service_instance::Opts { id, output }) => {
            service_instance::run(id, output).await?;
        }
        Subcommand::SshKey(ssh_key::Opts { id, output }) => {
            ssh_key::run(id, output).await?;
        }
    }
    Ok(())
}
