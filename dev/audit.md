# Dependency audit

`cargo audit` is run in CI (`ci.yml`). The allowlist lives at the repo root in
**`audit.toml`** (git-tracked) and **must be kept in sync with this page**.

## Current advisories allowed

| ID | Crate | Kind | Reason to allow |
| -- | ----- | ---- | --------------- |
| `RUSTSEC-2023-0089` | `atomic-polyfill` | unmaintained | Transitive dep, no exploitable path used by the crate. |
| `RUSTSEC-2024-0384` | `instant` | unmaintained | Transitive dep, no security impact on the crypto or file handling. |
| `RUSTSEC-2024-0388` | `derivative` | unmaintained | Transitive dev/tooling dep only. |

Rules:

* Do **not** extend the allowlist for direct or security-relevant dependencies.
* Remove an entry as soon as the dependency is no longer pulled in.
* Re-run `cargo audit` locally after any dependency change:
  `cargo audit` (uses `./audit.toml` automatically).

## Reference output (audited with the allowlist applied)

```bash
Crate:     atomic-polyfill
Version:   1.0.3
Warning:   unmaintained
Title:     atomic-polyfill is unmaintained
Date:      2023-07-11
ID:        RUSTSEC-2023-0089
URL:       https://rustsec.org/advisories/RUSTSEC-2023-0089

Crate:     derivative
Version:   2.2.0
Warning:   unmaintained
Title:     `derivative` is unmaintained; consider using an alternative
Date:      2024-06-26
ID:        RUSTSEC-2024-0388
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0388

Crate:     instant
Version:   0.1.13
Warning:   unmaintained
Title:     `instant` is unmaintained
Date:      2024-09-01
ID:        RUSTSEC-2024-0384
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0384

warning: 3 allowed warnings found
```