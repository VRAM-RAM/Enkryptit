# Architecture (English)

High-level design of the `eck` crate.

## Pipeline overview

On an encryption run, the flow is:

```
EnkryptitParams
   ├─ key_params   -> how the key is unlocked (password / file / OS keyring)
   ├─ compression  -> zstd | lz4 | xz | none | auto
   └─ parallelism  -> single | multi(auto) | multi(n) | auto

Path -> treatment::treat_object
   ├─ directory?  -> encrypt_folder_case + intern archive
   └─ file?       -> inspect: reads header (magic + version)
        ├─ .encky -> decrypt (single stream or folder archive)
        └─ other  -> encrypt (single file) or inspect-only
```

Both encrypt and decrypt cut the stream into 8 MiB chunks (`CHUNK_SIZE`), **compress then
encrypt** every chunk, and dispatch the work to a shared memory-locked worker pool.

## Concurrency model

* `EnkryptitPool<T: EnkryptitExecutable>` (`parallelism/`):
  * a synchronized **jobs** channel backed by `sync_channel(size * 2)`;
  * a **results** channel;
  * each `EnkryptitWorker` loops `recv -> execute -> send` until the channel closes.
* `EnkryptitJob` / `ChunkResult { index, data }` (`chunk_job.rs`) keep chunk ordering intact
  despite the unordered worker pool.
* `infer_parallelism` (`context.rs`) chooses `MultiThread` automatically based on file size
  thresholds and `available_parallelism()`.

## Format decision tree

The plaintext vs `.encky` decision happens in `treatment::treat_object`:
folders go to `encrypt_folder_case`; regular files read the header and branch on the magic
number and `is_folder_archive` flag (see [`format.md`](../user-guide/en/format.md)).

## Key material

Keys never stay in plain RAM:

* `EnkryptitKey { key: LockedKey, key_type }` — `LockedKey` uses `mlock` on the backing
  bytes and `zeroize` on drop.
* Resolution happens only at the moment of encrypt/decrypt (`key/resolve.rs`), never
  before: encrypting generates a missing key, decrypting **errors** if the key cannot be
  found (`KeyNotFoundInFile` / `KeyNotFoundInOs`).

See [Security](security.md).

## Error & reporting surface

* `EnkryptitError` → diagnostics through `miette` (labels, help, codes).
* `EnkryptitOutput` — typed effect returned everywhere; the frontends render it as
  `Success` / `Warning` / `Info` / `Error` banners.
* `DOC_URL` constant points at `doc/` so every diagnostic links to the documentation.

## Frontends

The core is UI-agnostic: the same `treatment`/`inspector` produce `EnkryptitOutput`,
rendered either by the **CLI** (`frontend/cli.rs`) or the **TUI**
(`frontend/tui.rs`, an iterator `save` loop over a mockable `TuiInput`).

## Related

* [Modules](modules.md) — exact file map of `eck/src`.
* [Testing](testing.md) — how the layers are verified.
* [Fuzzing](fuzzing.md) — parsing/roundtrip invariants.