# Enkryptit Test Suite 


### Test Structure

```
/test/
├── mod.rs                    # Root module with parallel execution config
│
├── unit/                     # Unit tests - isolated component testing
│   ├── compression.rs        # Zstd/Lz4/Xz compression roundtrip tests
│   ├── encryption_primitives.rs  # Key derivation & chunk crypto tests  
│   └── metadatas.rs          # Postcard serialization tests
│
├── integration/              # Integration tests - end-to-end workflows
│   ├── cli_tests.rs          # CLI argument parsing + params + roundtrips
│   ├── encryption_flow.rs    # Full encrypt/decrypt workflows + negative cases
│   ├── folder_encryption.rs  # Folder archive handling
│   └── mod.rs                # Module root with glob re-exports
│
└── mocks/                    # Test utilities and helpers
    ├── helpers.rs            # Runtime file generation, TestConfigGuard
    └── mod.rs                # Module root for mock exports
```


### Dependencies Added to Cargo.toml

Already configured with dev-dependencies:
```toml
[dev-dependencies]
tempfile = "3"           # Runtime temp file generation
assert_cmd = "2"         # CLI command testing  
predicates = "3"         # Output assertions
ctor = "0.2"             # Optional test setup macros
```

### Coverage Tooling

Configured for Rust built-in coverage:
- Uses `rustc --coverage` to generate reports in default `/target/` directory
- No external tools (cargo-tarpaulin) required
- Parallel execution enabled via `--test-threads=16` flag

## Test isolation: `TestConfigGuard`

CLI tests spawn real `eck` processes that read/write a params config file.
`mocks::helpers::TestConfigGuard` gives each test an isolated config:

```rust
let _guard = TestConfigGuard::new("PassWord", "Zstd");   // valid config
let _guard = TestConfigGuard::with_raw_content("...");   // arbitrary content
```

It points `ECK_CONFIG_PATH` at a temp file kept alive for the test duration,
and holds a global mutex so parallel tests never interleave env mutations.
Always hold the guard for the whole test, and always run the binary via
`Command::cargo_bin("eck")` (never PATH lookup, which can hit a stale
`~/.cargo/bin/eck`).

## Remaining work

- OS keyring roundtrip is `#[ignore]`d (`encrypt_decrypt_os_keytype_roundtrip`) until credential-store mocking is available
- Public keys module (`src/public_keys/`) is not yet implemented; tests to follow

## Known Issues

None — the suite is expected to be fully green (89 passed, 1 ignored).
