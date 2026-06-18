use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("no clipboard command found (install xclip, wl-clipboard, or pbcopy)")]
    NoCommand,
    #[error("clipboard command failed: {0}")]
    CommandFailed(String),
    #[error("failed to fork clear process: {0}")]
    ForkFailed(String),
}

fn clipboard_copy_cmd() -> Option<&'static str> {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        Some("wl-copy")
    } else if std::env::var("DISPLAY").is_ok() {
        Some("xclip")
    } else if cfg!(target_os = "macos") {
        Some("pbcopy")
    } else {
        None
    }
}

fn clipboard_clear_cmd() -> Option<String> {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        Some("wl-copy --clear".to_string())
    } else if std::env::var("DISPLAY").is_ok() {
        Some("xclip -selection clipboard < /dev/null".to_string())
    } else if cfg!(target_os = "macos") {
        Some("echo -n '' | pbcopy".to_string())
    } else {
        None
    }
}

pub fn copy_to_clipboard(value: &str) -> Result<(), ClipboardError> {
    let cmd = clipboard_copy_cmd().ok_or(ClipboardError::NoCommand)?;

    let result = match cmd {
        "wl-copy" => Command::new("wl-copy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(value.as_bytes())?;
                }
                child.wait()
            }),
        "xclip" => Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(value.as_bytes())?;
                }
                child.wait()
            }),
        "pbcopy" => Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(value.as_bytes())?;
                }
                child.wait()
            }),
        _ => return Err(ClipboardError::NoCommand),
    };

    result
        .map(|_| ())
        .map_err(|e| ClipboardError::CommandFailed(e.to_string()))
}

pub fn fork_clear_after(secs: u64) -> Result<(), ClipboardError> {
    let clear_cmd = clipboard_clear_cmd().ok_or(ClipboardError::NoCommand)?;

    // Fork a detached process that sleeps then clears
    let shell_cmd = format!("sleep {} && {}", secs, clear_cmd);

    Command::new("sh")
        .arg("-c")
        .arg(&shell_cmd)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| ClipboardError::ForkFailed(e.to_string()))?;

    Ok(())
}
