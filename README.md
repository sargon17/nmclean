# nmclean

A fast, safe CLI to find and delete `node_modules` folders.

`nmclean` helps you reclaim disk space by scanning a directory tree for `node_modules` and optionally deleting selected folders with an interactive prompt (or in bulk).

## Features
- Scan for `node_modules` with optional depth limit
- Interactive selection UI for safe deletion
- `--all` and `--yes` for non‑interactive cleanup
- `--dry-run` to preview without deleting
- Refuses to delete symlinks for safety

## Install
Requires Rust toolchain (stable).

### From GitHub
```bash
cargo install --git https://github.com/sargon17/nmclean
```

### From source
```bash
git clone https://github.com/sargon17/nmclean
cd nmclean
cargo build --release
# binary at target/release/nmclean
```

## Usage
```bash
nmclean scan --root .
nmclean scan --root . --max-depth 4

nmclean delete --root .
# non‑interactive cleanup
nmclean delete --root . --all --yes
# preview only
nmclean delete --root . --all --dry-run
```

### Commands
- `scan` — list all `node_modules` under a root path
- `delete` — remove selected `node_modules`

### Options
- `--root <path>` (default: `.`)
- `--max-depth <n>`
- `--all` (delete all found)
- `--dry-run`
- `--yes` (skip final confirmation)

## Examples
```bash
# list all node_modules under ~/Projects
nmclean scan --root ~/Projects

# interactively delete
nmclean delete --root ~/Projects

# clean everything under ~/Projects without prompts
nmclean delete --root ~/Projects --all --yes
```

## Safety
`nmclean` refuses to delete symlinked `node_modules` directories.

## License
MIT
