# Testing (English)

How the crate is verified.

## Layout

* `eck/test/mod.rs` — declares the single `eck_tests` harness (unit + integration).
* Unit tests live next to the code (`#[cfg(test)]` modules everywhere).
* Integration tests cover end-to-end encrypt/decrypt of files **and** folders.
* `eck/test/` also holds mocks and the messages `eck_tests` asserts on.

## Running

```bash
cargo xtask test
```

runs `cargo test --test=eck_tests` (all of them).

## Isolation

* Tests never touch the real user config: `ECK_CONFIG_PATH` (env var) redirects the
  parameters file to a temp dir, and `tempfile` crates throwaway directories.
* The OS-keyring integration test is an `#[ignore]`d test:
  `encrypt_decrypt_os_keytype_roundtrip` (`eck/test/integration/encryption_flow.rs:153`).
  It is run manually and by CI on macOS only (see [CI](README.md)).

## What is covered

1. **Key resolution** — password / key-file / OS-keyring roundtrips.
2. **Chunk flow** — `encrypt_stream`/`decrypt_stream` on overlapping-size inputs.
3. **Folder archives** — full directory roundtrip, preserving files, structure and
   permissions.
4. **Compression** — `zstd`, `lz4`, `xz`, `none` over byte ranges.
5. **CLI** — `assert_cmd`-style invocations of the built `eck` binary.
6. **TUI flows** — dialogs simulated through the mockable `TuiInput`.
7. **Errors** — wrong keys, truncated files, nonexistent paths surface the expected codes.

## New tests

Contributions should add both a fast unit test in the owning module and an integration test
in `eck/test/integration` when the behavior crosses module boundaries. Fuzz targets live in
[`fuzzing.md`](fuzzing.md).

## Related

* [Modules](modules.md) — where the tested code lives.
* [Fuzzing](fuzzing.md) — property-style coverage.