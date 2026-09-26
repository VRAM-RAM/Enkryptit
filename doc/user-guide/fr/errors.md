# Erreurs et sortie

Chaque exécution de `eck` rapporte par des bannières **EnkryptitOutput**. Les erreurs sont
rendues avec `miette` (rapports graphiques) et sont aussi écrites dans le fichier de log.

## Anatomie d'une erreur

| Champ | Signification |
| ----- | ------------- |
| **Message** | Description lisible de ce qui a échoué. |
| **Code** | Code stable, lisible par machine, `domaine::snake_case` (ou `none` sans code spécifique). |
| **Location** | Où l'erreur est survenue dans le code (si connu). |
| **Help** | Une aide pour résoudre l'erreur (si possible). |
| **Snippet** | L'invocation CLI contextuelle, ex. `eck <chemin>` (si disponible). |
| **URL** | Un lien vers la documentation : `doc/`. |

## Codes d'erreur

Tous les codes proviennent de `EnkryptitError::code()` dans `eck/src/errors.rs`. Ils sont
regroupés par domaine.

### `crypto::`

| Code | Variante | Message | Aide |
| ---- | -------- | ------- | ---- |
| `crypto::argon2error` | `Argon2Error` | argon2 password hash failed | La dérivation de clé Argon2 a échoué. Vérifiez votre mémoire disponible. |
| `crypto::encryption_decryption_failed` | `EncryptionError` | encryption/decryption failed | Vérifiez votre mot de passe/clé et que le fichier n'est pas corrompu. |
| `crypto::invalid_key_length` | `InvalidKeyLength` | invalid key length | La clé doit faire exactement 32 octets. |
| `crypto::invalid_key_type` | `InvalidKeyType` | Invalid key type. Found X, expected Y | Utilisez un type de clé pris en charge : Password, Os ou File. |
| `crypto::key_derivation_failed` | `KeyDerivationError` | key derivation error | Le mot de passe ou le fichier de clé semble invalide. |
| `crypto::key_not_found_in_file` | `KeyNotFoundInFile` | The file containing the key was not found | Placez votre fichier de clé au chemin configuré. |
| `crypto::key_not_found_in_os` | `KeyNotFoundInOs` | The Key couldn't be found in Os' keyring | Stockez d'abord la clé dans le trousseau (ex. Keychain / GNOME Keyring). |
| `crypto::error_with_os_keyring` | `KeyringError` | keyring error | Assurez-vous que votre trousseau système est déverrouillé. |

### `comp::`

| Code | Variante | Message | Aide |
| ---- | -------- | ------- | ---- |
| `comp::lz4_comp_failed` | `Lz4CompressionError` | Lz4 compressionError | Les données sont corrompues ou non compressées en Lz4. |
| `comp::zstd_comp_failed` | `ZstdError` | zstd error | Les données sont corrompues ou non compressées en Zstandard. |

> **Remarque :** `Lz4DecompressionError` ne porte actuellement pas de code spécifique
> (`none`).

### `ui::`

| Code | Variante | Message |
| ---- | -------- | ------- |
| `ui::command_not_found` | `CommandNotFound` | Command not found |
| `ui:tui_error` | `TuiError` | Error with the Tui |

> **Remarque :** le code TUI utilise un séparateur à deux points simple `ui:tui_error`
> (conservé tel quel dans la source).

### `format::`

| Code | Variante | Message | Aide |
| ---- | -------- | ------- | ---- |
| `format::corrupted_file` | `CorruptedFile` | Corrupted File | Attendez `eck recover <chemin>` pour de l'aide. |
| `format::directory_is_folder` | `DirectoryIsFolder` | Directory found is the directory of the folder | Vous ne pouvez pas traiter le dossier lui-même. |
| `format::metadata_reading_failed` | `FailedToReadMetadata` | Failed to read metadata of file | Vérifiez que le fichier existe et reste lisible. |

### `io::`

| Code | Variante | Message | Aide |
| ---- | -------- | ------- | ---- |
| `io::unexpected_eof` | `UnexpectedEof` | unexpected end of file | Le fichier est tronqué ; il est peut-être corrompu. |
| `io::home_directory_not_found` | `HomeNotFound` | home directory not found | Définissez la variable d'environnement HOME. |
| `io::misc_io_error` | `IoError` | io error | Vérifiez que le chemin existe et que vous avez les permissions nécessaires. |
| `io::incorrect_path` | `PathIsIncorrect` | Path is incorrect | Vérifiez que le chemin existe. |
| `io::file_not_found` | `FileError`/`SpecificFileError` | error while searching for the file | Vérifiez que le fichier existe et est lisible. |
| `io::file_is_a_symlink` | `FileIsASymLink` | File is a SymLink (shortcut) | Utilisez le chemin réel au lieu d'un lien symbolique. |
| `io::permissions_reading_failed` | `FilePermissionsParsingError` | Error while parsing file permissions | Les permissions de ce fichier n'ont pas pu être analysées. |

### `misc::`

| Code | Variante | Message | Aide |
| ---- | -------- | ------- | ---- |
| `misc::inspection_failed` | `InspectionError` | Inspection failed | Assurez-vous que le fichier existe et que son nom ne contient pas de caractères unicode invalides. |
| `misc::hex_decoding_failed` | `HexError` | hex decoding error | — |
| `misc::config_directory_error` | `ConfigError` | configuration error | Vérifiez l'emplacement et le format du fichier de configuration. |
| `misc::break` | `Break` | operation interrupted | — |

### Sans code (`none`)

Ces variantes rapportent `none` comme code :

* `PostcardError` — données sérialisées invalides (« The file seems corrupted or not in the
  Enkryptit! format. »).
* `StripPrefixError` — suppression de préfixe de chemin.
* `SendError`, `ReceiveError`, `InvalidWorkerCount` — mécanique du parallélisme.
* `MemoryLockError` — échec de `mlock` lors du verrouillage des clés en mémoire.
* `SerdeJsonError` — le fichier de paramètres/configuration n'est pas du JSON valide.
* `UnknownAction` — dispatch de commande inconnu.
* `Lz4DecompressionError` — échec de décompression lz4.

## Journaux

Un fichier de log à rotation quotidienne (`log.txt`) est écrit via `tracing` dans le
répertoire de configuration d'Enkryptit. Les erreurs sont toujours journalisées avant
d'être affichées.

## Liens

* [Format `.encky`](format.md) — le conteneur que ces erreurs analysent.