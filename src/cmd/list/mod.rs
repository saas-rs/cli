use clap::Parser;

pub(super) mod accounts;
pub(super) mod api_keys;
pub(super) mod issue_comments;
pub(super) mod issues;
pub(super) mod linked_accounts;
pub(super) mod output;
pub(super) mod plans;
pub(super) mod projects;
pub(super) mod service_instances;
pub(super) mod services;
pub(super) mod ssh_keys;
pub(super) mod transforms;

pub use output::{output, output_empty_result_of_unknown_schema};
pub use transforms::select_existing_cols;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::enum_variant_names)]
pub enum Subcommand {
    #[command(name = "accounts")]
    Accounts(accounts::Opts),

    #[command(name = "api-keys", alias = "apiKeys")]
    ApiKeys(api_keys::Opts),

    #[command(name = "issue-comments", alias = "issueComments")]
    IssueComments(issue_comments::Opts),

    #[command(name = "issues")]
    Issues(issues::Opts),

    #[command(name = "linked-accounts", alias = "linkedAccounts")]
    LinkedAccounts(linked_accounts::Opts),

    #[command(name = "plans")]
    Plans(plans::Opts),

    #[command(name = "projects")]
    Projects(projects::Opts),

    #[command(name = "services")]
    Services(services::Opts),

    #[command(name = "service-instances", alias = "serviceInstances")]
    ServiceInstances(service_instances::Opts),

    #[command(name = "ssh-keys", alias = "sshKeys")]
    SshKeys(ssh_keys::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Accounts(accounts::Opts { offset, limit, output }) => {
            accounts::run(offset, limit, output).await?;
        }
        Subcommand::ApiKeys(api_keys::Opts { offset, limit, output }) => {
            api_keys::run(offset, limit, output).await?;
        }
        Subcommand::IssueComments(issue_comments::Opts { offset, limit, output }) => {
            issue_comments::run(offset, limit, output).await?;
        }
        Subcommand::Issues(issues::Opts { offset, limit, output }) => {
            issues::run(offset, limit, output).await?;
        }
        Subcommand::LinkedAccounts(linked_accounts::Opts { offset, limit, output }) => {
            linked_accounts::run(offset, limit, output).await?;
        }
        Subcommand::Plans(plans::Opts { offset, limit, output }) => {
            plans::run(offset, limit, output).await?;
        }
        Subcommand::Projects(projects::Opts { offset, limit, output }) => {
            projects::run(offset, limit, output).await?;
        }
        Subcommand::Services(services::Opts { offset, limit, output }) => {
            services::run(offset, limit, output).await?;
        }
        Subcommand::ServiceInstances(service_instances::Opts { offset, limit, output }) => {
            service_instances::run(offset, limit, output).await?;
        }
        Subcommand::SshKeys(ssh_keys::Opts { offset, limit, output }) => {
            ssh_keys::run(offset, limit, output).await?;
        }
    }
    Ok(())
}
