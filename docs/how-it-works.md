# How it works

The `open` action is a small Rust program (sources under `src/`). When herdr invokes it, herdr sets `HERDR_PLUGIN_CONTEXT_JSON` with the invocation context. The binary reads that JSON, picks a directory, and either asks the local herdr client to open the workspace or launches `zed -n <dir>` directly on the server.

## Context

Environment variables set by herdr (see the [plugin docs](https://herdr.dev/docs/plugins/)):

| Variable                                                         | Used for                                                         |
| ---------------------------------------------------------------- | ---------------------------------------------------------------- |
| `HERDR_PLUGIN_CONTEXT_JSON`                                      | Source of the directory; must be present                         |
| `HERDR_BIN_PATH`                                                 | Used to detect and invoke the native `client.open_workspace` API |
| `HERDR_WORKSPACE_ID`                                             | Passed to `client.open_workspace`                                |
| `HERDR_PLUGIN_ID`, `HERDR_PLUGIN_ACTION_ID`, `HERDR_PLUGIN_ROOT` | Set by herdr but otherwise unused                                |

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

## Native client API

When `HERDR_BIN_PATH` and `HERDR_WORKSPACE_ID` are available, the plugin first runs:

```
herdr status server --json
```

If the response advertises `capabilities.client_open_workspace`, it invokes:

```
herdr open-workspace <workspace-id> --opener zed
```

The server forwards the request to the local herdr client. That client resolves its selected endpoint:

- Local endpoint: `zed -n <workspace-path>`
- SSH endpoint: `zed -n ssh://<target>/<workspace-path>`

This path supports `herdr --remote` and saved SSH machines without requiring Zed on the remote server. If the capability is absent, the plugin continues with the direct Zed lookup below.

## Zed lookup order

1. `ZED_BIN` from the environment, used as-is
2. `ZED_BIN` from `$HERDR_PLUGIN_CONFIG_DIR/.env` (dotenv-style `KEY=VALUE` lines; blank lines, `#` comments, a leading `export `, and matching quotes are handled)
3. `zed` on `PATH` (probed with `zed --version`)
4. Platform fallbacks, in order:

| OS      | Fallbacks                                                        |
| ------- | ---------------------------------------------------------------- |
| macOS   | `/usr/local/bin/zed`, `/Applications/Zed.app/Contents/MacOS/cli` |
| Linux   | `/usr/bin/zed`, `/usr/bin/zedit`, `/usr/bin/zeditor`             |
| Windows | none (PATH only)                                                 |

If nothing is found, the action exits 1 and prints a hint to install the CLI from Zed's command palette (`cli: install cli binary`), referencing the [Zed CLI reference](https://zed.dev/docs/reference/cli).

## JSON parsing

The context is parsed by [`serde_json`](https://crates.io/crates/serde_json) into a generic `Value`. The plugin then takes the first present, non-blank string along the resolution chain (`worktree.checkout_path` → `workspace_cwd` → `focused_pane_cwd`). Standard JSON escaping, including `\uXXXX` with surrogate pairs, is handled by serde_json, so Windows paths (`C:\\Users`) and non-ASCII directory names survive.

## Troubleshooting

- **`zed CLI not found`** — the CLI is not on PATH and not at a fallback location. Open Zed's command palette and run `cli: install cli binary` ([reference](https://zed.dev/docs/reference/cli)), or set `ZED_BIN` in the plugin config `.env`, then re-run the action.
- **`no directory in plugin context`** — the action ran without workspace context. Invoke it from inside a herdr workspace (e.g. via the keybinding), not from a bare shell.
- **`herdr open-workspace failed`** — the server advertised the native API but could not complete it. Check that a foreground client is attached and that `zed` is available on the machine running that client.
- **`zed exited with ...`** — Zed launched but returned a non-zero status; run `zed -n <dir>` manually to see the underlying error.
- Action logs: `herdr plugin log list --plugin open-in-zed`.
