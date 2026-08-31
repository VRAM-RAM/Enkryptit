# Enkryptit!
\
🔵 **Human-written**
\
\
![Enkryptit](https://img.shields.io/badge/enkryptit-rust-orange?style=for-the-badge&logo=rust)
![Version](https://img.shields.io/github/v/release/VRAM-RAM/enkryptit?style=for-the-badge)
![License](https://img.shields.io/badge/license-CeCILL--B%20%2F%20Apache%202.0-blue?style=for-the-badge)

**Enkryptit!** is a Rust-written cli-tool / interactive manager for file and folder encryption. 

> [!WARNING]
> This project is currently a work in progress. It is **not audited** for production security.

## Quick Start

### Prerequisites

* **Rust:** `1.75+` is recommended.
* A working native credential store (e.g., `libsecret` on Linux, native Keychain on macOS).

### Build and Launch

```bash
# Clone the repository
git clone https://github.com/VRAM-RAM/Enkryptit
cd Enkryptit

#Build :
cargo build --release

#Install in your current path :
cargo install --path .
```

> [!NOTE]
> The compilation time may be long, since I activated an agressive release profile.

## Usage & Commands

Enkryptit! can be run by two ways :

### CLI Tool

First, Enkryptit! can be run as a CLI tool. This are available commands :


| Command | Syntax | Description |
| --- | --- | --- |
| **Help** | `eck help` | Displays the available commands menu. |
| **Encrypt / Decrypt** | `eck /path/to/file` | Toggles encryption or decryption for the specified file. |
| **Encrypt / Decrypt with a password** | `eck /path/to/file -p mypassword` | Toggles encryption or decryption for the specified file, with a password. |
| **Encrypt / Decrypt multiple files and folders** | `eck /path/to/file /path/to/folder /path/to/folder/*` | Toggles encryption or decryption for the specified files and folders (compatible with -p) |
| **Open TUI** | `eck ui` or `eck` | Opens the TUI |
| **Show current params** | `eck parameters` or `eck params` | Shows current params |
| **Change compression** | `eck params (or parameters) -c (or --compression) compressiontype` | Changes compression algorithm. Available : zstd, lz4 and xz  |
| **Change key type** | `eck params (or parameters) -k (or --keytype) keytype` | Changes key type. Available : os, file and pwd (or password) |
| **Change parallelism type** | `eck params (or parameters) -p [parallelism-type] | Changes parallelism type. Available : `single`, `multi`, or `multi:threads_number` |

### TUI

Enkryptit! can also be run as a TUI. It offers a more "friendly" interface. To open the TUI, run :

```bash
eck
```
or

```bash
eck ui
```

You will see a menu, in which one you will be able to navigate :

```bash
Enkryptit
   Fast & Secure File Encryption Manager v2
? What do you want to do?  
> Encrypt/Decrypt file/folder
  Parameters
  Help
  Browse
  Exit
[↑↓ to move, enter to select, type to filter]
```

### Examples

For example, if you want to encrypt a file named `secrets.txt` in `/home/user/secrets/`, you just have to run :
```bash
eck /home/user/secrets/secrets.txt
```
Then, to decrypt the encrypted file as `secrets.txt.encky`, you run barely the same command :

```bash
eck /home/user/secrets/secrets.txt.encky
```

To encrypt a folder, you need to use the same `eck` command, but to specify a folder path :

```bash
eck /home/user/secrets/
```

This will encrypt your folder and the files it contains inside a file named `secrets.encky`. To decrypt it, simply run :

```bash
eck /home/user/secrets.encky
```

You can also encrypt / decrypt many files & folders with one command :

```bash
eck /home/user/secrets/* /home/user/secret.txt /lib/secret.bin
```

## Cryptographic Stack

Enkryptit! relies on those primitives for its architecture:

* **Encryption:** `XChaCha20-Poly1305` — Symmetric authenticated encryption (AEAD) with a 192-bit nonce.
* **Key Derivation:** `Argon2id` — The industry-standard password hashing algorithm, built to resist GPU/ASIC brute-force attacks.
* **Key Storage:** `Keyring` — Securely delegates key management to your operating system's native credential store (Keychain, Secret Service, etc.).
* **Data Serialization:** `Postcard` — A lightweight, efficient binary serialization format optimized for Rust.

But also : 

- `libc` for *mlock* and *munlock*
- `zeroize` for *Zeroize* and *Zeroizing<>*
- `lz4_flex` , `zstd` and `xz2` (compression)
- `rand` for *OsRng*

## Change parameters

You have the ability to choose your compression, key type and parallelism type with **Enkryptit!**. By default, the choosen parameters are :

- Keytype : Password
- Compression : Zstd
- Parallelism : Single (Thread)

## RoadMap

Next steps would be :

- parallelization
- benchmarks
- Doc + Dev Doc
- Make the interface more ergonomic (modify how metadata works...)
- Add a `Recovery` mode

## License

This project is dual-licensed under:

* **CeCILL-B License** (French law compliant, fully compatible with GNU GPL/Apache)
* **Apache License, Version 2.0**

Choose the one that best fits your needs.

## Contact

* **Developer:** Olruix ([VRAM-RAM](https://github.com/VRAM-RAM))
