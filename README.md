# Enkryptit!

🔵 **Human-written**

![Enkryptit](https://img.shields.io/badge/enkryptit-rust-orange?style=for-the-badge\&logo=rust) ![License](https://img.shields.io/badge/license-CeCILL--B%20%2F%20Apache%202.0-blue?style=for-the-badge)

**Enkryptit! - A fast, simple file & folder encryption manager, written in Rust**

> [!WARNING]
> This project is currently a work in progress. It is **not audited** for production security.

## Table of contents

* [Quick Start](#quick-start)
* [Usage & Commands](#usage--commands)
  * [CLI Tool](#cli-tool)
  * [TUI](#tui)
  * [Examples](#examples)
* [Why Enkryptit! ?](#why-enkryptit-)
* [Development](#development)
* [License](#license)

## Quick Start

### Prerequisites

* **Rust:** `1.75+` is recommended.
* A working native credential store (e.g. `libsecret` on Linux, native Keychain on macOS).

### Build and Launch

```bash
# Clone the repository
git clone https://github.com/VRAM-RAM/Enkryptit

cd Enkryptit/

# Build
cargo xtask build --release

# Or Install
cargo xtask install
```

> [!NOTE]
> Compilation may take some time because Enkryptit uses an aggressive release profile.

## Usage & Commands

Enkryptit! can be used in two ways: as a CLI tool or as a TUI.

### CLI Tool

These are the available commands:

| Command                               | Syntax                           | Description                                                                            |
| ------------------------------------- | -------------------------------- | -------------------------------------------------------------------------------------- |
| **Encrypt / Decrypt**                 | `eck <path>`                     | Automatically encrypts plaintext or decrypts `.encky` files.                           |
| **Encrypt / Decrypt with a password** | `eck <path> -p <password>`       | Encrypts or decrypts using the specified password.                                     |
| **Open TUI**                          | `eck ui` or `eck`                | Opens the interactive TUI.                                                             |
| **Show current parameters**           | `eck parameters` or `eck params` | Shows the current parameters.                                                          |
| **Change compression**                | `eck params -c <type>`           | Changes the compression algorithm. Available: `zstd`, `lz4`, `xz`, `none`, and `auto`. |
| **Change key type**                   | `eck params -k <type>`           | Changes the key type. Available: `os`, `file`, and `pwd` / `password`.                 |
| **Change parallelism**                | `eck params -p <type>`           | Changes the parallelism mode. Available: `single`, `multi`, or `multi:<threads>`.      |
| **Inspect**                           | `eck inspect <path>`             | Inspects a file or archive without encrypting or decrypting it.                        |

> [!TIP]
> `eck <path>` and `eck inspect <path>` support multiple paths:
>
> `eck <path1> <path2>`
>
> `eck inspect myfolder/*`

### TUI

Enkryptit! also provides an interactive terminal user interface.

Launch it with:

```bash
eck
```

or:

```bash
eck ui
```

It will open :
```bash
Enkryptit
   Fast & Simple File Encryption Manager v0.0.3
? What do you want to do?  
> Browse
  Parameters
  Help
  Exit
[↑↓ to move, enter to select, type to filter]
```

The Tui allows you to directly **browse** your files / folders.

### Examples

Encrypt a file:

```bash
eck /home/user/secrets/secrets.txt
```

This creates:

```text
secrets.txt.encky
```

To decrypt it:

```bash
eck /home/user/secrets/secrets.txt.encky
```

Encrypt a folder:

```bash
eck /home/user/secrets/
```

This creates:

```text
secrets.encky
```

To decrypt it:

```bash
eck /home/user/secrets.encky
```

Multiple files and folders can be processed at once:

```bash
eck /home/user/secrets/* /home/user/secret.txt /lib/secret.bin
```

Inspect a file without decrypting it:

```bash
eck inspect /home/user/my_secret.txt
```

## Why Enkryptit! ?

* **Modern primitives** — XChaCha20-Poly1305 provides authenticated encryption, protecting encrypted data against unauthorized modification as well as unauthorized access.

* **Flexible key management** — use a password, the OS keyring, or a dedicated key file depending on your workflow.

* **Built for large files** — chunked streaming encryption avoids requiring the entire file to be loaded into memory.

* **Automatic performance optimization** — compression and parallelism can be infered automatically based on the file and its size.

* **First-class folder encryption** - encrypt an entire directory into a single `.encky` archive while preserving its structure and Unix permissions.

* **Security-conscious secret handling** — keys are memory-locked and zeroized when no longer needed.

* **CLI + TUI** — use a fast, scriptable CLI or an interactive terminal interface without needing separate applications.

* **Transparent inspection** — `eck inspect` can examine encrypted files and archives without decrypting their contents.

* **Extensively tested** — 199+ tests cover cryptography, key handling, tamper detection, folder encryption, multithreading, CLI behavior, and TUI flows.

**In short:** Enkryptit combines **strong authenticated encryption, flexible key management, efficient large-file processing, automatic compression and parallelism, and a usable CLI/TUI** in one native Rust application.

## Development

**Enkryptit!** contains :
- Its binary, in `eck/`
- `cargo xtask` for launching tests, fuzzy testing and rustdoc

You can follow the development in [**PROGRESSION.md**](./PROGRESSION.md).

You can also help the development by [**contributing**](CONTRIBUTING.md)!

## License

This project is dual-licensed under:

* **CeCILL-B License** (French-law compliant)
* **Apache License, Version 2.0**

Choose the license that best fits your needs.
