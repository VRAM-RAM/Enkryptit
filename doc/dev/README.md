# Development guide (English)

Everything a contributor needs to hack on **Enkryptit!**.

> [!NOTE]
> This folder is written in English by convention. The user-facing guides are bilingual in
> [`../user-guide`](../README.md).

## Tooling

The workspace helperscripts live in `xtask/`:

| Command | What it does |
| ------- | ------------ |
| `cargo xtask build` / `cargo xtask build --release` | Build `eck` (debug/release). |
| `cargo xtask install` | `cargo install --path ./eck`. |
| `cargo xtask test` | Runs the test binary `--test=eck_tests`. |
| `cargo xtask fuzz` | Lists fuzz targets (`cargo fuzz list`). |
| `cargo xtask fuzz <target> [FLAGS]` | Runs a libFuzzer target; extra flags are passed through (e.g. `-runs=1000`, `-max_total_time=60`). |
| `cargo xtask fmt` | `cargo fmt`. |
| `cargo xtask doc` | Opens rustdoc with private items. |

## Repository layout

```
Cargo.toml            # workspace (eck, xtask; eck/fuzz excluded)
eck/                  # the crate & binary
eck/src/              # see dev/modules.md
eck/test/             # unit + integration tests + mocks
eck/fuzz/             # cargo-fuzz targets (nightly toolchain)
xtask/                # dev helper binary
doc/                  # this documentation
dev/                  # benchmarking & audit notes
PROGRESSION.md        # day-by-day dev log
CONTRIBUTING.md       # contribution rules
```

## Quality bar

* `cargo fmt` clean.
* `cargo clippy -- -D warnings` clean.
* `cargo xtask test` green (unit + integration + TUI flows).
* `cargo doc --no-deps -D warnings` clean.
* `cargo audit` green (see `audit.toml` allowlist).
* The fuzz targets keep running without crashing (see [fuzzing](fuzzing.md)).
* RustDoc comments on public items.

## CI

The repo runs GitHub Actions:

* `ci.yml` — fmt / clippy / tests / rustdoc / audit on `dev` + `main` (push & PR).
* `ci.yml` — bounded fuzz smoke + `#[ignore]`d OS-keyring roundtrip.
* `release.yml` — tagged `v*` release matrix (draft release).
* `dependabot.yml` — weekly dependency updates.

See the workflow files under `.github/workflows`.

## Related

* [Architecture](architecture.md) — module design & data flows.
* [Modules](modules.md) — `eck/src` map.
* [Testing](testing.md) — test suite structure & isolation.
* [Fuzzing](fuzzing.md) — libFuzzer targets.
* [Security](security.md) — key handling, parameters & the audit allowlist.