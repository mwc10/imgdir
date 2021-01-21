use anyhow::{Context, Result};
use std::{
    fs,
    io::{self},
    path::PathBuf,
};

fn usage() {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("{} [study id] [study name]", env!("CARGO_BIN_NAME"));
    println!("  [study id]      Study ID (will prompt if not entered)");
    println!("  [study name]    Study Name (will prompt if not entered)");
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let id = args
        .next()
        .map(Ok)
        .unwrap_or_else(|| prompt("Enter Study ID"))?;
    if id == "-h" || id == "--help" {
        usage();
        return Ok(());
    }
    let name = args
        .next()
        .map(Ok)
        .unwrap_or_else(|| prompt("Enter Study Name"))?;

    create_image_directory(&id, &name)
}

fn prompt(label: &str) -> Result<String> {
    let mut buf = String::with_capacity(100);
    println!("{}:", label);
    io::stdin()
        .read_line(&mut buf)
        .with_context(|| format!("reading stdin for: {}", label))?;
    Ok(buf.trim().into())
}

fn create_image_directory(id: &str, name: &str) -> Result<()> {
    let mut dir = PathBuf::from(format!("{}_{}", id, name));
    dir.push("assay_images");
    dir.push(id);
    fs::create_dir_all(&dir).context("creating assay_images directory")?;
    dir.pop();
    dir.pop();

    dir.push("assay_thumbs");
    dir.push(id);
    fs::create_dir_all(&dir).context("creating assay_thumbs directory")?;
    dir.pop();
    dir.pop();

    dir.push("orig_metadata");
    fs::create_dir_all(&dir).context("creating orig_metadata directory")
}
