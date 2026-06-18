use crate::config;
use crate::output::table;
use crate::project::resolver;
use crate::session;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load_config();
    let vault_exists = config::vault_path().exists();

    // Session info
    let session_status = match session::load_session() {
        Ok(Some(s)) => {
            if s.is_valid() {
                let remaining = s.remaining_secs();
                let hours = remaining / 3600;
                let mins = (remaining % 3600) / 60;
                if hours > 0 {
                    format!("active ({}h{}m remaining)", hours, mins)
                } else {
                    format!("active ({}m remaining)", mins)
                }
            } else {
                "expired".to_string()
            }
        }
        Ok(None) => "none".to_string(),
        Err(_) => "error reading session".to_string(),
    };

    // Project info
    let project_info = match resolver::load_current_project() {
        Ok(p) => format!("{} / {}", p.project, p.environment),
        Err(_) => "none (use 'envsec use <project> <env>')".to_string(),
    };

    let rows = vec![
        table::StatusRow {
            field: "Vault".to_string(),
            value: if vault_exists {
                config::vault_path().display().to_string()
            } else {
                "not initialized".to_string()
            },
        },
        table::StatusRow {
            field: "Session".to_string(),
            value: session_status,
        },
        table::StatusRow {
            field: "Session Duration".to_string(),
            value: format!("{}s ({}h)", cfg.session.duration_secs, cfg.session.duration_secs / 3600),
        },
        table::StatusRow {
            field: "Clipboard Clear".to_string(),
            value: format!("{}s ({}m)", cfg.clipboard.clear_after_secs, cfg.clipboard.clear_after_secs / 60),
        },
        table::StatusRow {
            field: "Active Project".to_string(),
            value: project_info,
        },
    ];

    table::print_table(&rows, Some("envsec status"));
    println!();

    Ok(())
}
