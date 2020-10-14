use std::env;
use std::process;

use anyhow::{Context, Result};
use chrono::NaiveDateTime as DateTime;

fn git_stuff() -> Result<()> {
    let dir = env::var("CARGO_MANIFEST_DIR")?;
    let repo = git2::Repository::open(dir)?;
    let git_describe = repo
        .describe(&git2::DescribeOptions::new().describe_tags())?
        .format(Some(
            &git2::DescribeFormatOptions::new().dirty_suffix("-dirty"),
        ))?;
    let commit = repo.head()?.peel_to_commit()?;
    let commit_date = DateTime::from_timestamp_opt(commit.time().seconds(), 0)
        .context("invalid timestamp")?
        .format("%Y-%m-%d");
    println!("cargo:rustc-env=GIT_DESCRIBE={}", git_describe);
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", commit.id());
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", commit_date);
    Ok(())
}

fn rustc_stuff() -> Result<()> {
    let stdout = String::from_utf8(
        process::Command::new(env::var("RUSTC")?)
            .arg("--verbose")
            .arg("--version")
            .output()?
            .stdout,
    )?;
    for line in stdout.lines().skip(1) {
        let mut it = line.splitn(2, ": ");
        let key = it.next().unwrap();
        let value = it.next().unwrap();
        println!(
            "cargo:rustc-env=RUSTC_VERSION_{}={}",
            key.replace('-', "_").replace(' ', "_").to_uppercase(),
            value,
        )
    }
    Ok(())
}

fn main() -> Result<()> {
    // Pass through the target that we are building.
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET")?);

    // Try get the Rustc information but ignore if we failed.
    rustc_stuff().ok();

    // Try get Git information but ignore if we failed.
    git_stuff().ok();

    Ok(())
}
