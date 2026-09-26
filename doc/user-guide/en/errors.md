# Errors & output

Every `eck` run reports through **EnkryptitOutput** banners. Errors are rendered with
`miette` (fancy graphical reports) and are also written to the log file.

> TODO: verify the exact visual rendering and whether severity shows the ✖ bullet, the
> location line, or an embedded code block depending on kind.

## Anatomy of an error

| Field | Meaning |
| ----- | ------- |
| **Message** | Description of what failed. |
| **Code** | Stable machine-readable code, `domain::snake_case` (or `none` if no specific code). |
| **Location** | Where in the code base it happened (when known). |
| **Help** | A remediation hint (when available). |
| **Snippet** | Contextual CLI invocation, e.g. `eck <path>` for example (when available). |
| **URL** | A link to the documentation: `doc/`. |

## Error codes

All codes come from `EnkryptitError::code()` in `eck/src/errors.rs`. Codes are grouped by
domain.

### `crypto::`

| Code | Error variant | Message | Help |
| ---- | ------------- | ------- | ---- |
| `crypto::argon2error` | `Argon2Error` | argon2 password hash failed | Key derivation with Argon2 failed. Check your available memory. |
| `crypto::encryption_decryption_failed` | `EncryptionError` | encryption/decryption failed | Check your password/key and make sure the file is not corrupted. |
| `crypto::invalid_key_length` | `InvalidKeyLength` | invalid key length | The key must be exactly 32 bytes long. |
| `crypto::invalid_key_type` | `InvalidKeyType` | Invalid key type. Found X, expected Y | Use a supported key type: Password, Os or File. |
| `crypto::key_derivation_failed` | `KeyDerivationError` | key derivation error | The password or key file seems invalid. |
| `crypto::key_not_found_in_file` | `KeyNotFoundInFile` | The file containing the key was not found | Place your key file at the configured path. |
| `crypto::key_not_found_in_os` | `KeyNotFoundInOs` | The Key couldn't be found in Os' keyring | Store the key in the OS keyring (e.g. keychain / GNOME Keyring) first. |
| `crypto::error_with_os_keyring` | `KeyringError` | keyring error | Make sure your OS keyring is unlocked. |

### `comp::`

| Code | Error variant | Message | Help |
| ---- | ------------- | ------- | ---- |
| `comp::lz4_comp_failed` | `Lz4CompressionError` | Lz4 compressionError | The data is corrupted or not Lz4-compressed. |
| `comp::zstd_comp_failed` | `ZstdError` | zstd error | The data is corrupted or not Zstandard-compressed. |

> **Note:** `Lz4DecompressionError` currently carries no specific code (`none`).

### `ui::`

| Code | Error variant | Message |
| ---- | ------------- | ------- |
| `ui::command_not_found` | `CommandNotFound` | Command not found |
| `ui:tui_error` | `TuiError` | Error with the Tui |

> **Note:** the TUI code uses a single-colon separator `ui:tui_error` (kept as-is in the source).

### `format::`

| Code | Error variant | Message | Help |
| ---- | ------------- | ------- | ---- |
| `format::corrupted_file` | `CorruptedFile` | Corrupted File | Wait for `eck recover <path>` for help. |
| `format::directory_is_folder` | `DirectoryIsFolder` | Directory found is the directory of the folder | You can't treat the current folder itself. |
| `format::metadata_reading_failed` | `FailedToReadMetadata` | Failed to read metadata of file | Check that the file still exists and is readable. |

### `io::`

| Code | Error variant | Message | Help |
| ---- | ------------- | ------- | ---- |
| `io::unexpected_eof` | `UnexpectedEof` | unexpected end of file | The file is truncated; it may be corrupted. |
| `io::home_directory_not_found` | `HomeNotFound` | home directory not found | Set the HOME environment variable. |
| `io::misc_io_error` | `IoError` | io error | Check that the path exists and that you have the required permissions. |
| `io::incorrect_path` | `PathIsIncorrect` | Path is incorrect | Check that the path exists. |
| `io::file_not_found` | `FileError`/`SpecificFileError` | error while searching for the file | Check that the file exists and is readable. |
| `io::file_is_a_symlink` | `FileIsASymLink` | File is a SymLink (shortcut) | Use the real path instead of a symbolic link. |
| `io::permissions_reading_failed` | `FilePermissionsParsingError` | Error while parsing file permissions | The permissions of this file could not be parsed. |

### `misc::`

| Code | Error variant | Message | Help |
| ---- | ------------- | ------- | ---- |
| `misc::inspection_failed` | `InspectionError` | Inspection failed | Please ensure that the file exists, and that its name does not contain wrong unicode character. |
| `misc::hex_decoding_failed` | `HexError` | hex decoding error | — |
| `misc::config_directory_error` | `ConfigError` | configuration error | Check the config file location and format. |
| `misc::break` | `Break` | operation interrupted | — |

### Uncoded (`none`)

The following variants report `none` as their code:

* `PostcardError` — invalid serialized data ("The file seems corrupted or not in the Enkryptit! format.").
* `StripPrefixError` — path prefix stripping.
* `SendError`, `ReceiveError`, `InvalidWorkerCount` — parallelism plumbing.
* `MemoryLockError` — `mlock` failure while locking keys in RAM.
* `SerdeJsonError` — the parameters/config file is not valid JSON.
* `UnknownAction` — unknown command dispatch.
* `Lz4DecompressionError` — lz4 decompression failure.

## Logs

A daily-rotated log file (`log.txt`) is written through `tracing` into the Enkryptit config
directory. Errors are always traced before being displayed.

## Related

* [`.encky` format](format.md) — the container these errors parse.