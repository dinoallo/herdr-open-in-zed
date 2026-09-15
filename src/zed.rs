//! Locating the Zed CLI.

use std::path::Path;
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

const ZED_BIN_ENV: &str = "ZED_BIN";
const PLUGIN_CONFIG_DIR_ENV: &str = "HERDR_PLUGIN_CONFIG_DIR";

/// An explicit `ZED_BIN` wins: first the process environment, then a
/// `ZED_BIN=...` line in the plugin config `.env`. Otherwise bare `zed` on
/// PATH, then the known install locations.
pub(crate) fn find_zed() -> Option<String> {
    if let Some(configured) = configured_zed_bin() {
        return Some(configured);
    }
    if probe("zed") {
        return Some("zed".to_string());
    }
    FALLBACKS.iter().find(|p| probe(p)).map(|p| p.to_string())
}

/// Error message for when no CLI is found, pointing at the `ZED_BIN` override.
pub(crate) fn not_found_message() -> String {
    match std::env::var_os(PLUGIN_CONFIG_DIR_ENV) {
        Some(dir) => format!(
            "zed CLI not found; install it from Zed's command palette \
             (\"cli: install cli binary\") or set ZED_BIN in {}; \
             see https://zed.dev/docs/reference/cli",
            Path::new(&dir).join(".env").display()
        ),
        None => "zed CLI not found; install it from Zed's command palette \
                 (\"cli: install cli binary\") or set ZED_BIN; \
                 see https://zed.dev/docs/reference/cli"
            .to_string(),
    }
}

fn configured_zed_bin() -> Option<String> {
    if let Ok(raw) = std::env::var(ZED_BIN_ENV) {
        if let Some(value) = non_blank(&raw) {
            return Some(value.to_string());
        }
    }
    let dir = std::env::var_os(PLUGIN_CONFIG_DIR_ENV)?;
    let content = std::fs::read_to_string(Path::new(&dir).join(".env")).ok()?;
    env_value(&content, ZED_BIN_ENV)
}

fn non_blank(value: &str) -> Option<&str> {
    let value = value.trim();
    if value.is_empty() { None } else { Some(value) }
}

/// Reads `key` from dotenv-style text: blank lines and `#` comments are
/// skipped, a leading `export ` and matching quotes around the value are
/// stripped, and blank assignments are ignored. No expansion is performed.
fn env_value(content: &str, key: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let line = line
            .strip_prefix("export ")
            .map(str::trim_start)
            .unwrap_or(line);
        let (name, value) = line.split_once('=')?;
        if name.trim() != key {
            return None;
        }
        non_blank(unquote(value.trim())).map(str::to_string)
    })
}

fn unquote(value: &str) -> &str {
    for quote in ['"', '\''] {
        if let Some(inner) = value
            .strip_prefix(quote)
            .and_then(|rest| rest.strip_suffix(quote))
        {
            return inner;
        }
    }
    value
}

fn probe(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_value_reads_plain_assignment() {
        assert_eq!(
            env_value("ZED_BIN=zedit\n", ZED_BIN_ENV).as_deref(),
            Some("zedit")
        );
    }

    #[test]
    fn env_value_skips_comments_and_other_keys() {
        let content = "# comment\nOTHER=1\n\nZED_BIN=/opt/zed\n";
        assert_eq!(env_value(content, ZED_BIN_ENV).as_deref(), Some("/opt/zed"));
    }

    #[test]
    fn env_value_strips_export_and_quotes() {
        assert_eq!(
            env_value("export ZED_BIN='zedit'", ZED_BIN_ENV).as_deref(),
            Some("zedit")
        );
        assert_eq!(
            env_value(r#"ZED_BIN="/usr/bin/zedit""#, ZED_BIN_ENV).as_deref(),
            Some("/usr/bin/zedit")
        );
    }

    #[test]
    fn env_value_skips_blank_assignments() {
        assert_eq!(
            env_value("ZED_BIN=\nZED_BIN=zedit\n", ZED_BIN_ENV).as_deref(),
            Some("zedit")
        );
        assert_eq!(env_value("ZED_BIN=\"\"", ZED_BIN_ENV), None);
    }

    #[test]
    fn env_value_ignores_malformed_lines() {
        assert_eq!(
            env_value("ZED_BIN\nZED_BIN=zedit\n", ZED_BIN_ENV).as_deref(),
            Some("zedit")
        );
        assert_eq!(env_value("not a dotenv", ZED_BIN_ENV), None);
    }

    #[test]
    fn env_value_returns_none_when_absent() {
        assert_eq!(env_value("OTHER=1\n", ZED_BIN_ENV), None);
    }

    #[test]
    fn unquote_leaves_unmatched_quotes() {
        assert_eq!(unquote("\"zedit"), "\"zedit");
        assert_eq!(unquote("'zedit"), "'zedit");
    }
}
