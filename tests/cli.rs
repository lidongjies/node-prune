use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn node_modules_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("node-prune")?;
    cmd.assert()
        .stderr(predicates::str::contains("access node_modules error"));
    Ok(())
}
