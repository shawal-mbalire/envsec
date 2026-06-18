use clap::Parser;

mod cli;
mod clipboard;
mod config;
mod dotenv;
mod output;
mod project;
mod session;
mod sync;
mod vault;

use cli::{Cli, Commands};
use output::colors;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_DATE: &str = env!("ENVSEC_BUILD_DATE");
const GIT_HASH: &str = env!("ENVSEC_GIT_HASH");

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Skip version check for these commands
    let skip_version_check = matches!(
        cli.command,
        Commands::Version | Commands::Update | Commands::Devices | Commands::Sync { .. }
    );

    let result = match cli.command {
        Commands::Init => cli::init::run(),
        Commands::Auth { ref duration } => {
            cli::auth::run(duration.as_deref())
        }
        Commands::Status => cli::status::run(),
        Commands::Set {
            ref key,
            ref value,
        } => cli::set::run(key, value.as_deref()),
        Commands::Get { ref key, show } => cli::get::run(key, show),
        Commands::List { all } => cli::list::run(all),
        Commands::Rm { ref key } => cli::rm::run_rm(key),
        Commands::Rename {
            ref old_key,
            ref new_key,
        } => cli::rm::run_rename(old_key, new_key),
        Commands::Import {
            ref file,
            ref project,
            ref env,
        } => cli::import::run_import(file, project.as_deref(), env.as_deref()),
        Commands::Export { ref file, raw } => {
            cli::export::run_export(file.as_deref(), raw)
        }
        Commands::Run { ref command } => cli::run::run(command),
        Commands::Projects => cli::projects::run_projects(),
        Commands::Use {
            ref project,
            ref environment,
        } => cli::projects::run_use(project, environment.as_deref()),
        Commands::RmProject { ref project } => cli::projects::run_rm_project(project),
        Commands::Version => {
            println!("envsec {} ({} {})", CURRENT_VERSION, BUILD_DATE, GIT_HASH);
            Ok(())
        }
        Commands::Update => cli::update::run(),
        Commands::Devices => cli::devices::run().await,
        Commands::Sync {
            ref project,
            ref env,
            push,
            pull,
        } => {
            cli::sync::run(project.as_deref(), env.as_deref(), push, pull).await
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", colors::error("Error:"), e);
        std::process::exit(1);
    }

    // Non-blocking version check after command completes
    if !skip_version_check {
        check_for_update();
    }
}

fn check_for_update() {
    use std::time::Duration;

    // Quick check with short timeout, don't block the user
    let url = format!(
        "https://api.github.com/repos/shawal-mbalire/envsec/releases/latest"
    );

    let Ok(response) = ureq::get(&url)
        .set("User-Agent", &format!("envsec/{}", CURRENT_VERSION))
        .timeout(Duration::from_secs(2))
        .call()
    else {
        return; // Silently fail
    };

    let Ok(body) = response.into_string() else {
        return;
    };

    let Ok(release) = serde_json::from_str::<serde_json::Value>(&body) else {
        return;
    };

    let Some(tag) = release.get("tag_name").and_then(|t| t.as_str()) else {
        return;
    };

    let latest = tag.strip_prefix('v').unwrap_or(tag);

    if latest != CURRENT_VERSION {
        eprintln!(
            "\n{} New version available: v{} -> v{}  Run {} to update.",
            colors::dim("[update]"),
            CURRENT_VERSION,
            colors::success(latest),
            colors::bold("envsec update")
        );
    }
}
