//! herdr plugin action that opens the current workspace in the Zed editor.
//!
//! Reads `HERDR_PLUGIN_CONTEXT_JSON`, resolves a directory from the plugin
//! context, and launches `zed -n <dir>` in a new window. The Zed CLI
//! detaches by itself, so plain status polling is enough.
//!
//! Plugin docs: https://herdr.dev/docs/plugins/
//! Zed CLI reference: https://zed.dev/docs/reference/cli

mod context;
mod zed;

use std::process::Command;

fn main() {
    if let Err(e) = run() {
        eprintln!("open-in-zed: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let ctx = match std::env::var("HERDR_PLUGIN_CONTEXT_JSON") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return Err(
                "HERDR_PLUGIN_CONTEXT_JSON is not set; invoke this action from herdr".to_string(),
            );
        }
    };

    let dir = context::dir_from_context(&ctx)?;

    let zed = zed::find_zed().ok_or_else(zed::not_found_message)?;

    let status = Command::new(&zed)
        .args(["-n", &dir])
        .status()
        .map_err(|e| format!("failed to launch {zed}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("zed exited with {status}"))
    }
}
