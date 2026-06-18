use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use crate::output::colors;

const REPO: &str = "shawal-mbalire/envsec";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("HTTP request failed: {0}")]
    Http(String),
    #[error("JSON parse error: {0}")]
    Json(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("no release found")]
    NoRelease,
    #[error("no asset found for your platform")]
    NoAsset,
    #[error("update failed: {0}")]
    UpdateFailed(String),
}

#[derive(serde::Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn detect_target() -> &'static str {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    match (os, arch) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        _ => "unknown",
    }
}

fn parse_version(tag: &str) -> &str {
    tag.strip_prefix('v').unwrap_or(tag)
}

fn versions_equal(a: &str, b: &str) -> bool {
    a == b
}

fn fetch_latest_release() -> Result<Release, UpdateError> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let body: String = ureq::get(&url)
        .set("User-Agent", &format!("envsec/{}", CURRENT_VERSION))
        .call()
        .map_err(|e| UpdateError::Http(e.to_string()))?
        .into_string()
        .map_err(|e| UpdateError::Http(e.to_string()))?;

    serde_json::from_str(&body).map_err(|e| UpdateError::Json(e.to_string()))
}

fn find_asset<'a>(release: &'a Release, target: &str) -> Option<&'a Asset> {
    let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
    let pattern = format!("{}-{}.{}", release.tag_name, target, ext);

    release.assets.iter().find(|a| a.name == pattern)
}

fn download_to_temp(url: &str) -> Result<PathBuf, UpdateError> {
    let tmpdir = env::temp_dir().join("envsec-update");
    fs::create_dir_all(&tmpdir)?;

    let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
    let archive_path = tmpdir.join(format!("envsec.{}", ext));

    let response = ureq::get(url)
        .set("User-Agent", &format!("envsec/{}", CURRENT_VERSION))
        .call()
        .map_err(|e| UpdateError::Http(e.to_string()))?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| UpdateError::Http(e.to_string()))?;

    fs::write(&archive_path, &bytes)?;
    Ok(archive_path)
}

fn extract_and_install(archive_path: &PathBuf) -> Result<PathBuf, UpdateError> {
    let tmpdir = archive_path.parent().unwrap();
    let binary_name = if cfg!(windows) { "envsec.exe" } else { "envsec" };

    // Extract
    if cfg!(windows) {
        // Use PowerShell to extract zip
        std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    archive_path.display(),
                    tmpdir.display()
                ),
            ])
            .output()
            .map_err(|e| UpdateError::UpdateFailed(e.to_string()))?;
    } else {
        std::process::Command::new("tar")
            .args(["xzf", &archive_path.to_string_lossy(), "-C", &tmpdir.to_string_lossy()])
            .output()
            .map_err(|e| UpdateError::UpdateFailed(e.to_string()))?;
    }

    let extracted = tmpdir.join(binary_name);
    if !extracted.exists() {
        return Err(UpdateError::UpdateFailed(
            "binary not found in archive".to_string(),
        ));
    }

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&extracted, fs::Permissions::from_mode(0o755))?;
    }

    Ok(extracted)
}

fn replace_binary(new_binary: &PathBuf) -> Result<(), UpdateError> {
    let current = env::current_exe().map_err(|e| UpdateError::UpdateFailed(e.to_string()))?;

    // On Unix, rename is atomic
    #[cfg(unix)]
    {
        fs::rename(new_binary, &current)
            .map_err(|e| UpdateError::UpdateFailed(e.to_string()))?;
    }

    // On Windows, copy over (may fail if binary is locked)
    #[cfg(windows)]
    {
        fs::copy(new_binary, &current)
            .map_err(|e| UpdateError::UpdateFailed(e.to_string()))?;
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} Checking for updates... (current: v{})",
        colors::dim("*"),
        CURRENT_VERSION
    );

    let release = fetch_latest_release()?;
    let latest = parse_version(&release.tag_name);

    if versions_equal(latest, CURRENT_VERSION) {
        println!(
            "{} envsec is already up to date (v{})",
            colors::success("OK"),
            CURRENT_VERSION
        );
        return Ok(());
    }

    println!(
        "{} New version available: v{} -> v{}",
        colors::warning("!"),
        CURRENT_VERSION,
        colors::success(latest)
    );

    let target = detect_target();
    if target == "unknown" {
        return Err(format!(
            "No binary available for your platform ({} {})",
            env::consts::OS,
            env::consts::ARCH
        )
        .into());
    }

    let asset = find_asset(&release, target).ok_or(UpdateError::NoAsset)?;

    println!(
        "{} Downloading {}...",
        colors::dim("*"),
        asset.name
    );

    let archive_path = download_to_temp(&asset.browser_download_url)?;
    let new_binary = extract_and_install(&archive_path)?;

    replace_binary(&new_binary)?;

    // Cleanup
    let _ = fs::remove_file(&archive_path);
    let _ = fs::remove_dir_all(archive_path.parent().unwrap());

    println!(
        "{} Updated to v{}",
        colors::success("OK"),
        latest
    );

    Ok(())
}
