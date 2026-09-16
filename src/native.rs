//! Calls the native `client.open_workspace` API when the running server supports it.

use std::process::Command;

const HERDR_BIN_ENV: &str = "HERDR_BIN_PATH";
const WORKSPACE_ID_ENV: &str = "HERDR_WORKSPACE_ID";

/// Returns `Ok(true)` when the native API handled the request, `Ok(false)` when
/// the running herdr binary or server predates the API, and `Err` when a
/// supported API attempt failed.
pub(crate) fn try_open_workspace() -> Result<bool, String> {
    let Some(herdr) = non_blank_env(HERDR_BIN_ENV) else {
        return Ok(false);
    };
    let Some(workspace_id) = non_blank_env(WORKSPACE_ID_ENV) else {
        return Ok(false);
    };

    let status = Command::new(&herdr)
        .args(["status", "server", "--json"])
        .output()
        .map_err(|e| format!("failed to inspect {herdr}: {e}"))?;
    if !status.status.success() || !supports_client_open_workspace(&status.stdout) {
        return Ok(false);
    }

    let output = Command::new(&herdr)
        .args(["open-workspace", &workspace_id, "--opener", "zed"])
        .output()
        .map_err(|e| format!("failed to invoke {herdr} open-workspace: {e}"))?;
    if output.status.success() {
        return Ok(true);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let detail = if !stderr.trim().is_empty() {
        stderr.trim()
    } else if !stdout.trim().is_empty() {
        stdout.trim()
    } else {
        "no error output"
    };
    Err(format!(
        "herdr open-workspace failed with {}: {detail}",
        output.status
    ))
}

fn non_blank_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn supports_client_open_workspace(json: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(json)
        .ok()
        .and_then(|value| {
            value
                .pointer("/capabilities/client_open_workspace")
                .and_then(serde_json::Value::as_bool)
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_the_server_capability() {
        assert!(supports_client_open_workspace(
            br#"{"status":"running","capabilities":{"client_open_workspace":true}}"#
        ));
        assert!(!supports_client_open_workspace(
            br#"{"status":"running","capabilities":{"health_check":true}}"#
        ));
        assert!(!supports_client_open_workspace(b"not json"));
    }
}
