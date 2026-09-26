# Installation

This page explains how to build and install **Enkryptit!** on your machine.

> **TODO (fr/en sync):** adapt specific remarks (macOS Gatekeeper, Linux `libsecret`, Windows store) as platform-specific notes get written.

## Requirements

* **Rust:** `1.85+` (the declared `rust-version` of the workspace). Edition `2024` is used.
* A native credential store for `Key type = OS Keyring`:
  * **Linux:** `libsecret` (GNOME Keyring / KSecretService).
  * **macOS:** the system Keychain.
  * **Windows:** the Windows Credential Manager.

## Build

```bash
git clone https://github.com/VRAM-RAM/Enkryptit
cd Enkryptit/

# Debug build
cargo xtask build

# Release build (recommended)
cargo xtask build --release
```

The `eck` binary is produced at `target/release/eck` (or `target/debug/eck`).

> **Note:** the release profile is aggressive (`lto = "fat"`, `codegen-units = 1`,
> `panic = "abort"`). Compilation can take a while on the first build.

## Install

```bash
cargo xtask install
```

This runs `cargo install --path ./eck` and puts `eck` on `$PATH`.

## First run

```bash
eck              # opens the TUI
eck --help       # CLI help
eck --version    # prints enkryptit version
```

See [CLI](cli.md) and [TUI](tui.md) for usage.