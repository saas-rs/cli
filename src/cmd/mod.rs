use clap::Parser;

pub(super) mod create;
pub(super) mod delete;
pub(super) mod generate;
pub(super) mod get;
pub(super) mod history;
pub(super) mod initialize;
pub(super) mod list;
pub(super) mod login;
pub(super) mod logout;
pub(super) mod use_identity_provider;
pub(super) mod use_storage_provider;
pub(super) mod version;

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Create a new manageable model
    #[command(name = "create")]
    Create(create::Opts),

    /// Delete a single manageable model
    #[command(name = "delete")]
    Delete(delete::Opts),

    /// Generate various things like XIDs, GUIDs, Models, Controllers, Resources, and Services
    #[command(name = "generate", alias = "g")]
    Generate(generate::Opts),

    /// Get a single manageable model
    #[command(name = "get")]
    Get(get::Opts),

    /// List the history of mutate actions
    #[command(name = "history")]
    History(history::Opts),

    /// Initialize the current git workspace with a new SaaS
    #[command(name = "init")]
    Initialize(initialize::Opts),

    /// List the manageable models
    #[command(name = "list")]
    List(list::Opts),

    /// Launch a browser to perform a login; listen on a dynamic port to wait for the browser's completion callback
    #[command(name = "login")]
    Login(login::Opts),

    /// Remove your access token (session) from the config file in your home dir
    #[command(name = "logout")]
    Logout(logout::Opts),

    #[command(name = "use-identity-provider", alias = "useIdentityProvider")]
    UseIdentityProvider(use_identity_provider::Opts),

    #[command(name = "use-storage-provider", alias = "useStorageProvider")]
    UseStorageAdapter(use_storage_provider::Opts),

    /// Display the build version
    #[command(name = "version")]
    Version(version::Opts),
}
