use std::{error::Error, process::Command};

const TAILWIND_CSS_CLI: &str = "./tailwind/tailwindcss";
const TAILWIND_CSS_FILE: &str = "./tailwind/tailwind.css";

fn main() -> Result<(), Box<dyn Error>> {
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
