use std::{error::Error, process::Command};
use time::{OffsetDateTime, format_description};

const TAILWIND_CSS_CLI: &str = "./tailwind/tailwindcss";
const TAILWIND_CSS_FILE: &str = "./tailwind/tailwind.css";

fn main() -> Result<(), Box<dyn Error>> {
    // Set build timestamp
    let now = OffsetDateTime::now_utc();
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC")?;
    let timestamp = now.format(&format)?;
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Set git commit hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    println!("cargo:rerun-if-changed={TAILWIND_CSS_FILE}");
    println!("cargo:rerun-if-changed=src/views/");

    let output = Command::new(TAILWIND_CSS_CLI)
        .args([
            "-i",
            TAILWIND_CSS_FILE,
            "-o",
            "./static/styles.css",
            "--minify",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "failed to execute `tailwindcss`:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
