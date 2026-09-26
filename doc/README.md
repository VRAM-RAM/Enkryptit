# Documentation

## User guide / Guide utilisateur

| Topic / Sujet | English | Français |
| ------------- | ------- | -------- |
| Installation | [en](user-guide/en/installation.md) | [fr](user-guide/fr/installation.md) |
| Command line / Ligne de commande | [en](user-guide/en/cli.md) | [fr](user-guide/fr/cli.md) |
| Terminal UI / Interface | [en](user-guide/en/tui.md) | [fr](user-guide/fr/tui.md) |
| Errors & output / Erreurs | [en](user-guide/en/errors.md) | [fr](user-guide/fr/errors.md) |
| `.encky` format | [en](user-guide/en/format.md) | [fr](user-guide/fr/format.md) |

> [!NOTE]
> English and French user guides are kept in sync (mirror trees in `user-guide/en` and
> `user-guide/fr`).

## Developer guide / Guide développeur

English only (by convention). See [dev/README.md](dev/README.md).

* [dev/architecture.md](dev/architecture.md) — module design & data flows.
* [dev/modules.md](dev/modules.md) — the `eck/src` map.
* [dev/testing.md](dev/testing.md) — test suite & isolation.
* [dev/fuzzing.md](dev/fuzzing.md) — libFuzzer targets.
* [dev/security.md](dev/security.md) — key handling & audit allowlist.

## Format

More on the container: magic `ENK1`, version `3`, postcard metadata,
XChaCha20-Poly1305 per 8 MiB chunk — see [format](user-guide/en/format.md)
(and the French version [format](user-guide/fr/format.md)).
