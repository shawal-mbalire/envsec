use std::process::Command;

fn main() {
    // Embed build date as YYYYMMDD
    let date = std::env::var("BUILD_DATE").unwrap_or_else(|_| {
        let output = Command::new("date")
            .args(["+%Y%m%d"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        output
    });
    println!("cargo:rustc-env=ENVSEC_BUILD_DATE={}", date);

    // Embed git hash
    let hash = std::env::var("GIT_SHA").unwrap_or_else(|_| {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        output
    });
    println!("cargo:rustc-env=ENVSEC_GIT_HASH={}", hash);

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
