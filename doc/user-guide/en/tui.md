# Terminal UI

The interactive terminal interface lets you browse and treat files and folders without
remembering command-line flags.

## Launching

```bash
eck        # no argument -> TUI
eck ui     # explicit TUI command
```

## Main menu

```
Enkryptit
   Fast & Simple File Encryption Manager v0.0.3
? What do you want to do?
> Browse
  Parameters
  Help
  Exit
```

| Entry | Description |
| ----- | ----------- |
| **Browse** | Pick files, folders, or both, then encrypt / decrypt / inspect them. |
| **Parameters** | Configure compression, key type and parallelism. |
| **Help** | Shows the available commands. |
| **Exit** | Leaves the TUI. |

## Browse panel

```
Browser Panel
? What do you want to Browse?
> Browse Files
  Browse Folders
  Browse both Files & Folders
  Back to main menu
```

After picking file(s)/folder(s) with the system picker, each selection runs the standard
treatment (encrypt `.encky` / decrypt plaintext).

> **Note:** the TUI prompts for the password (and cipher) interactively when needed.

## Parameters panel

```
Parameters Panel
? What do you want to configure?
> Change compression type
  Change key type
  Change parallelism type
  Show current parameters
  Back to main menu
```

Choices are persisted to the same `config.json` as the CLI (see [Parameters](../cli.md)).

## Related

* [CLI](cli.md) — the equivalent command-line interface, including the parameters mapping.

> **TODO:** add a screenshot of the actual TUI rendering.