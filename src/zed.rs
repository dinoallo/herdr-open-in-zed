//! Locating the Zed CLI.

use std::process::{Command, Stdio};

#[cfg(target_os = "macos")]
const FALLBACKS: &[&str] = &[
    "/usr/local/bin/zed",
    "/Applications/Zed.app/Contents/MacOS/cli",
];

#[cfg(target_os = "linux")]
const FALLBACKS: &[&str] = &["/usr/bin/zed", "/usr/bin/zedit", "/usr/bin/zeditor"];

#[cfg(target_os = "windows")]
const FALLBACKS: &[&str] = &[];

/// Bare `zed` on PATH first, then the known install locations.
pub(crate) fn find_zed() -> Option<String> {
    if probe("zed") {
        return Some("zed".to_string());
    }
    FALLBACKS.iter().find(|p| probe(p)).map(|p| p.to_string())
}

fn probe(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}
