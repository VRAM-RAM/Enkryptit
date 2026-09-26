# Fuzzing (English)

The parser and roundtrip invariants are fuzzed with **cargo-fuzz + libFuzzer**.

## Targets

Defined in `eck/fuzz/` (its own workspace with a `nightly` toolchain):

| Target | Invariant |
| ------ | --------- |
| `decompress` | Feeding arbitrary bytes to the decompression pipeline never panics or hangs. |
| `compress_roundtrip` | Compress → decompress of arbitrary input yields the original bytes (or fails cleanly). |
| `metadata` | Parsing arbitrary postcard metadata/headers never panics. |

## Running locally

```bash
cargo xtask fuzz               # list targets
cargo xtask fuzz decompress    # run a target (all extra args pass through)
cargo xtask fuzz decompress -runs=1000 -max_total_time=60
```

> [!NOTE]
> `cargo-fuzz` requires the **nightly** Rust toolchain (`eck/fuzz/rust-toolchain.toml`).
> The `fuzz/` workspace is excluded from the main workspace's CI compile jobs.

## Corpus

* Interesting inputs are kept in `eck/fuzz/corpus/<target>/` (gitignored).
* Reinvesting the corpus on every target run keeps the interesting cases around.

## In CI

A **bounded smoke job** runs each target with `-max_total_time=60`; a crash, timeout or
panic fails the job (**blocking**): see [CI](README.md).

## Adding a target

1. Create `eck/fuzz/fuzz_targets/<name>.rs` using the standard `fuzz_target!` macro and
   one of the crate's entry points.
2. Declare it in `eck/fuzz/Cargo.toml` under `[[bin]]`.
3. Document the invariant above in the doc-comment.

## Related

* [Testing](testing.md) — deterministic tests.
* [Modules](modules.md) — the parsing functions being fuzzed.