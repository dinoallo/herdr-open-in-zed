# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Native `client.open_workspace` support, allowing remote herdr workspaces to open in the Zed client over SSH.

## 0.1.0

### Added

- `open` action that resolves the workspace directory from the herdr plugin context (`worktree.checkout_path` → `workspace_cwd` → `focused_pane_cwd`) and opens it in a new Zed window via `zed -n <dir>`.
- Zed CLI lookup: `zed` on `PATH`, then known install locations on macOS and Linux.
- Documentation: README, how it works, development guide, contributing guide.
- CI: fmt, clippy, tests, and release build on Linux, macOS, and Windows; release workflow publishing binaries for `v*` tags.
