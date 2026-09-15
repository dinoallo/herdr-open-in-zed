# open-in-zed

![open-in-zed](assets/banner.svg)

herdr plugin that opens the current workspace in the Zed editor.

[herdr](https://herdr.dev) is a terminal workspace manager; [Zed](https://zed.dev) is a code editor. This plugin bridges them: one keypress in a herdr workspace and Zed opens the same directory in a new window via `zed -n <path>`. It targets the herdr plugin API v1 and runs on Linux and macOS (Windows untested).

## Requirements

- herdr >= 0.9.0
- Rust toolchain (the plugin builds itself on install)
- The Zed CLI. Install it from Zed's command palette: run `cli: install cli binary`. See the [Zed CLI reference](https://zed.dev/docs/reference/cli). On Linux, distribution packages usually ship it as `zed`, `zedit` (Gentoo), or `zeditor` (Debian/Ubuntu).

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

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/development.md](docs/development.md).

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

[MIT](LICENSE)
