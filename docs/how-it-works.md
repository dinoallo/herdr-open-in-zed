# How it works

The `open` action is a small Rust program (sources under `src/`). When herdr invokes it, herdr sets `HERDR_PLUGIN_CONTEXT_JSON` with the invocation context. The binary reads that JSON, picks a directory, finds the `zed` CLI, and runs `zed -n <dir>` (open in a new window). Zed's CLI detaches by itself, so the plugin just waits for the launch to succeed and exits.

## Context

Environment variables set by herdr (see the [plugin docs](https://herdr.dev/docs/plugins/)):

| Variable                                                                           | Used for                                                           |
| ---------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `HERDR_PLUGIN_CONTEXT_JSON`                                                        | Source of the directory and nothing else; must be present          |
| `HERDR_PLUGIN_ID`, `HERDR_PLUGIN_ACTION_ID`, `HERDR_PLUGIN_ROOT`, `HERDR_BIN_PATH` | Set by herdr but unused by this plugin (no herdr callbacks needed) |

Fields read from the context JSON (all optional; absent when null or blank):

| Field                    | Meaning                                                      |
| ------------------------ | ------------------------------------------------------------ |
| `worktree.checkout_path` | Checkout directory of the worktree the workspace is based on |
| `workspace_cwd`          | Workspace working directory (may be a subdirectory)          |
| `focused_pane_cwd`       | Working directory of the focused pane                        |

## Path resolution

```
worktree.checkout_path > workspace_cwd > focused_pane_cwd > error
```

`worktree.checkout_path` wins because `workspace_cwd` may be a subdirectory the user has `cd`-ed into; opening Zed there would show the subdirectory as the project root instead of the repository. A field counts only if it is a present, non-blank string; a non-string or whitespace-only value counts as absent and the chain falls through to the next candidate.

If no field yields a directory, the action exits 1 with `no directory in plugin context`. This happens when the action is invoked outside a workspace context.

## Zed lookup order

1. `zed` on `PATH` (probed with `zed --version`)
2. Platform fallbacks, in order:

| OS      | Fallbacks                                                        |
| ------- | ---------------------------------------------------------------- |
| macOS   | `/usr/local/bin/zed`, `/Applications/Zed.app/Contents/MacOS/cli` |
| Linux   | `/usr/bin/zed`, `/usr/bin/zedit`, `/usr/bin/zeditor`             |
| Windows | none (PATH only)                                                 |

If nothing is found, the action exits 1 and prints a hint to install the CLI from Zed's command palette (`cli: install cli binary`), referencing the [Zed CLI reference](https://zed.dev/docs/reference/cli).

## JSON parsing

The context is parsed by [`serde_json`](https://crates.io/crates/serde_json) into a generic `Value`. The plugin then takes the first present, non-blank string along the resolution chain (`worktree.checkout_path` → `workspace_cwd` → `focused_pane_cwd`). Standard JSON escaping, including `\uXXXX` with surrogate pairs, is handled by serde_json, so Windows paths (`C:\\Users`) and non-ASCII directory names survive.

## Troubleshooting

- **`zed CLI not found`** — the CLI is not on PATH and not at a fallback location. Open Zed's command palette and run `cli: install cli binary` ([reference](https://zed.dev/docs/reference/cli)), then re-run the action.
- **`no directory in plugin context`** — the action ran without workspace context. Invoke it from inside a herdr workspace (e.g. via the keybinding), not from a bare shell.
- **`zed exited with ...`** — Zed launched but returned a non-zero status; run `zed -n <dir>` manually to see the underlying error.
- Action logs: `herdr plugin log list --plugin open-in-zed`.
