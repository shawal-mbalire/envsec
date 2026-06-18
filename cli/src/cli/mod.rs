pub mod auth;
pub mod export;
pub mod get;
pub mod import;
pub mod init;
pub mod list;
pub mod projects;
pub mod rm;
pub mod run;
pub mod set;
pub mod status;
pub mod update;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envsec", about = "Local-first encrypted secret manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize vault and set master passphrase
    Init,
    /// Authenticate with passphrase to start a session
    Auth {
        /// Session duration (e.g., 2h, 30m, 3600s)
        #[arg(long)]
        duration: Option<String>,
    },
    /// Show session status and active project info
    Status,
    /// Set a secret in the current project/environment
    Set {
        /// Secret key name
        key: String,
        /// Secret value (omit to enter interactively)
        value: Option<String>,
    },
    /// Copy a secret to clipboard (auto-clears after 2 minutes)
    Get {
        /// Secret key name
        key: String,
        /// Show masked value to stdout instead of clipboard
        #[arg(long)]
        show: bool,
    },
    /// List secrets in the current project/environment
    List {
        /// List all projects and environments
        #[arg(long)]
        all: bool,
    },
    /// Remove a secret
    Rm {
        /// Secret key name
        key: String,
    },
    /// Rename a secret
    Rename {
        /// Current key name
        old_key: String,
        /// New key name
        new_key: String,
    },
    /// Import a .env file into the current project
    Import {
        /// Path to .env file
        file: String,
        /// Target project name
        #[arg(long)]
        project: Option<String>,
        /// Target environment name
        #[arg(long, short = 'e')]
        env: Option<String>,
    },
    /// Export secrets as a .env file
    Export {
        /// Output file path (stdout if omitted)
        #[arg(long, short)]
        file: Option<String>,
        /// Show actual values (requires confirmation)
        #[arg(long)]
        raw: bool,
    },
    /// Run a command with secrets injected as environment variables
    Run {
        /// Command and arguments to run
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<String>,
    },
    /// List all projects
    Projects,
    /// Switch active project and environment
    Use {
        /// Project name
        project: String,
        /// Environment name (default: "default")
        environment: Option<String>,
    },
    /// Remove a project and all its secrets
    RmProject {
        /// Project name
        project: String,
    },
    /// Show version
    Version,
    /// Check for updates and install latest version
    Update,
}
