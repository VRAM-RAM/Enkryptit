# Command Line

`eck` is a scriptable CLI. If no path is given, `eck` opens the [TUI](tui.md).

## Global options

| Option | Description |
| ------ | ----------- |
| `eck <path>...` | Encrypts a plaintext path or decrypts a `.encky` path (file or folder). Multiple paths accepted. |
| `-p, --password <pass>` | Provide the password directly instead of prompting. |
| `eck ui` | Open the TUI. |
| `eck inspect <path>...` | Inspect paths without encrypting/decrypting. |

## Commands

| Command | Meaning |
| ------- | ------- |
| `eck <path>` | Encrypts a plaintext file / folder, or decrypts an `.encky`. |
| `eck <path> -p <password>` | Same, with the password given on the command line. |
| `eck ui` | Opens the interactive TUI. |
| `eck parameters` or `eck params` | Shows the current parameters. |
| `eck params -c <algo>` | Changes the compression algorithm. Values: `zstd`, `lz4`, `xz`, `none`, `auto`. |
| `eck params -k <type>` | Changes the key type (`params -k` alias `--keytype`). Values: `os`, `file`, `pwd`/`password`. |
| `eck params -p <mode>` | Changes the parallelism mode. Values: `single`, `multi`, `multi:<threads>` or `auto` |
| `eck inspect <path>` | Inspects a file or archive without encrypting or decrypting it. |

`parameters` and `params` are equivalent; both accept `-c`, `-k` (`--keytype`) and `-p`.
The `parameters` variant also exposes the visible alias `kt` for `--keytype`.
Without any flag they print the current parameters instead.

> **Note:** `eck params -k os` and `eck parameters --keytype os` both set the key type.

## Behavior

```
eck /home/user/secrets/secrets.txt      -> secrets.txt.encky
eck /home/user/secrets.txt.encky        -> secrets.txt
```

* Multiple paths are processed in one call:

  ```
  eck /home/user/secrets/* /home/user/secret.txt /lib/secret.bin
  eck inspect myfolder/*
  ```

* A **folder** is turned into a single **archive** `<folder>.encky` (see [format](format.md)).
* Encryption/decryption is chosen automatically: `.encky` files decrypt, everything else encrypts.

> [!NOTE]
> When you encrypt an object with a **keyfile**, the file is stored in `HOME/private_keys`.

> [!WARNING]
> Encryption with **keyring** is unstable for now.

## Examples

```bash
# Encrypt a file with a password
eck secrets.txt -p 's3cret!'

# Encrypt a folder (produces secrets.encky)
eck secrets/

# Decrypt it back
eck secrets.encky

# Change compression (persisted in the config)
eck params -c zstd

# Inspect without decrypting
eck inspect secrets.txt.encky
```

## Exit behavior

Success and failure are reported through **EnkryptitOutput** banners (see [errors](errors.md)).
CLI parameter errors exit with a non-zero status and a warning.