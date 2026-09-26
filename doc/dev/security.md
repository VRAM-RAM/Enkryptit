# Security (English)

Threat model, key handling and known caveats.

## Key handling

* Keys are stored in **memory-locked** buffers: `LockedKey` keeps the backing bytes
  `mlock`ed and `zeroize`s them on drop (page-fault hardening, no copy on disk).
* A key exists only for the duration of an encrypt/decrypt run; it is re-resolved each time,
  never cached across CLI invocations.
* `EnkryptitKey` always knows its provenance (`key_type`) and validates lengths during
  resolve (`InvalidKeyLength` for bad sizes).

## Key derivation

Passwords go through **Argon2id** (RFC 9106-ish, 128 MiB, 3 iterations, 1 lane, 32-byte
output) with a 16-byte random salt per encryption:

```
password (+salt) -> argon2id -> 32-byte key
```

## Secrets storage

| Key type | Location |
| -------- | -------- |
| `Password` | prompted, or `-p` from the shell (visible in `ps` if used on the CLI — prefer environment or stdin). |
| `Pwd256` | salted argon2id (above). |
| `File` | `~/private_keys/enkryptit_<stem>` (per-file key files). |
| `Os` | OS keyring: service `ENKRYPTIT`, user `enkryptit<filename>`, value hex-encoded. |

> **Note:** the `-p` flag value may be visible in process listings/history. Favor the
> interactive prompt or the OS keyring for sensitive environments.

## Format / integrity

* AEAD (XChaCha20-Poly1305) authenticates every chunk; tampering fails decryption with an
  auth error rather than silently corrupting data.
* `ENK1END` marker detects truncated single-file streams; folder archives validate their
  trailing `FolderMetadata`.
* Magic + version gate every parse; unknown versions error instead of guessing.

## Known caveats

* Zip-style folder architectures currently rely on file `relative_path`s; paths with
  problematic Unicode are rejected during inspection (`misc::inspection_failed`).
* `panic = "abort"` in release builds jumps straight to abort (no destructors run) — the
  keyed buffers are only guaranteed zeroized on the normal drop path in debug/threads.
* The auditing stance below does not silence general advisories, only three transitives.

## Dependency audit

`cargo audit` runs in CI. The `audit.toml` at the repo root allowlists exactly three
**unmaintained-transitive** advisories, all from non-crypto indirect dependencies:

| Advisory | Crate | Reason to allow |
| -------- | ----- | --------------- |
| `RUSTSEC-2023-0089` | `atomic-polyfill` | unmaintained transitive dev/tooling dep; not in the security-critical path. |
| `RUSTSEC-2024-0384` | `instant` | unmaintained transitive; no exploitable path used by the crate. |
| `RUSTSEC-2024-0388` | `derivative` | unmaintained transitive; unsafely used only by an indirect dev dependency. |

`dev/audit.md` mirrors this allowlist and records the reasoning. Do not extend the allowlist
for direct or security-relevant dependencies.

## Reporting

Open an issue / private advisory upstream. Until a fix lands, prefer NOT to weaken the
allowlist above.