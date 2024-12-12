use std::process::{Command, Stdio};

use crate::template::Day;

pub fn handle(day: Day, release: bool, dhat: bool) {
    let mut args = vec!["run".to_string(), "--bin".to_string(), day.to_string()];

    if dhat {
        args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ])
    } else if release {
        args.push("--release".to_string());
    }

    args.push("--".to_string());

    let mut cmd = Command::new("cargo")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }
}