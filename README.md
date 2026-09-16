# open-in-zed

![open-in-zed](assets/banner.svg)

herdr plugin that opens the current workspace in the Zed editor.

[herdr](https://herdr.dev) is a terminal workspace manager; [Zed](https://zed.dev) is a code editor. This plugin bridges them: one keypress in a herdr workspace and Zed opens the same directory in a new window via `zed -n <path>`. It targets the herdr plugin API v1 and runs on Linux and macOS (Windows untested).

## Requirements

- herdr >= 0.9.0
- Rust toolchain (the plugin builds itself on install)
- The Zed CLI on the machine running the local Herdr client. Install it from Zed's command palette: run `cli: install cli binary`. See the [Zed CLI reference](https://zed.dev/docs/reference/cli). On Linux, distribution packages usually ship it as `zed`, `zedit` (Gentoo), or `zeditor` (Debian/Ubuntu). Older Herdr builds that lack the native client API instead require Zed on the machine running the plugin.

On Herdr builds that ship the `type = "open_workspace"` keybinding and `[[openers]]` config, this plugin is optional: you can bind the built-in command and define a Zed opener locally instead. The plugin remains the way to open workspaces from older Herdr builds, and it stays useful when the opener needs logic beyond an argv template.

## Install

```
herdr plugin install alexeyco/herdr-open-in-zed
```

## Usage

Bind a key to the action in your herdr config:

```toml
[[keys.command]]
key = "prefix+shift+o"
type = "plugin_action"
command = "open-in-zed.open"
description = "Open workspace in Zed"
```

Any chord works; see the [herdr keyboard docs](https://herdr.dev/docs/keyboard/) for picking one that your terminal doesn't swallow. You can also invoke the action from the CLI:

```
herdr plugin action invoke open-in-zed.open
```

The action resolves a directory from the plugin context: the worktree checkout path first (so a focused subdirectory still opens the repo root), then the workspace cwd, then the focused pane cwd. Details in [docs/how-it-works.md](docs/how-it-works.md).

## Remote workspaces

On Herdr builds that expose `client.open_workspace`, the plugin asks the
foreground client to open the workspace. The local client resolves its selected
endpoint and runs either `zed -n <local-path>` or
`zed -n ssh://<target>/<remote-path>`, so Zed can open the remote project over
SSH without installing Zed on the remote server.

The plugin falls back to running `zed -n <path>` on the Herdr server when the
native API is unavailable. Remote fallback therefore still requires a Zed CLI
on that server and is mainly useful when the server also has a graphical
session.

## Configuration

The plugin finds the Zed CLI on its own: `zed` on `PATH`, then the known install locations (`/usr/bin/zed`, `/usr/bin/zedit`, `/usr/bin/zeditor` on Linux). To point it at a specific binary, set `ZED_BIN` to the command or absolute path (no arguments):

```
echo 'ZED_BIN=zedit' >> "$(herdr plugin config-dir open-in-zed)/.env"
```

`ZED_BIN` is read from the environment first, then from that `.env` file (a simple `KEY=VALUE` subset: `#` comments, blank lines, optional `export ` prefix, and matching quotes are handled). When set, the automatic lookup is skipped and the configured value is used as-is.

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/development.md](docs/development.md).

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

[MIT](LICENSE)
