# Le format `.encky`

Un conteneur chiffré est écrit sous la forme **`<base>.encky`**. Il existe **deux** types de
conteneurs :

1. un **fichier** chiffré (objet simple),
2. une **archive de dossier** (un dossier entier dans un seul `.encky`).

Tout est sérialisé avec **postcard** ; la version est stockée dans l'en-tête
(constante `VERSION`, actuellement `3` — la 3ᵉ nightly de la première stable).

## Primitives communes

* Magic number : `0x45 0x4E 0x4B 0x31` = `ENK1`.
* Taille de chunk : `CHUNK_SIZE = 8 MiB`.
* Chiffrement : **XChaCha20-Poly1305** (nonce de 24 octets, AEAD).
* Clé de 32 octets (depuis un mot de passe / un fichier de clé / le trousseau système).

## Nonce principal et nonces dérivés

Chaque objet (ou entrée d'archive) possède un **nonce principal** stocké dans ses
métadonnées. Pour chaque bloc `step` (donc à chaque étape), un nonce dérivé est construit :

```
new_nonce[0..16]  = master_nonce[0..16]
new_nonce[16..24] = step.to_le_bytes()   # 8 octets
```

## Flux chiffré 

Dans le flux chiffré, le 'payload' est une séquence de blocs (précédés par leur taille) suivie d'un marqueur final, 'ENK1END' :

```
[ chunk_len:u32 LE ][ fragment_compressé ]
[ chunk_len:u32 LE ][ fragment_compressé ]
...
[ "ENK1END" ]        # 7 octets littéraux
```

* Chaque bloc est d'abord **compressé**, puis **chiffré** avec le nonce dérivé de son `step`.
* La partie compressée tient dans un buffer pouvant atteindre `CHUNK_SIZE`, mais chaque
  fragment écrit est précédé de sa longueur réelle

## Disposition d'un fichier simple

```
offset 0 : header_len:u8                       # longueur de l'en-tête postcard (>= 1 octet)
          ArchiveHeader := magic[4] + version:u8 + is_folder_archive:false + meta_len:u32
          MetaDatas     := key_type + compression + nonce[24]
          puis le flux chiffré (voir ci-dessus), suivi par ENK1END
```

`payload_offset` pour le déchiffrement = `1 + header_len + meta_len`.

## Disposition d'une archive de dossier (v2+)

```
octet 0                        : HEADER_REGION_SIZE:u8 (= 64)
octets 1 .. 65                 : ArchiveHeader, rembourré à zéro sur 64 octets
                                 (magic, version, is_folder_archive:true, meta_len)
octets 65 ..                   : un flux chiffré par fichier, séquentiellement,
                                 chacun avec son propre file_nonce (voir FileEntry)
fin du fichier                 : FolderMetadata (postcard)
                                 = key_type + entries[ FileEntry ]
```

* `HEADER_REGION_SIZE` garde la zone d'en-tête fixe afin que l'en-tête puisse être remis à
  jour en place sans changer la taille de la zone.
* Les entrées sont chiffrées les unes après les autres ; chaque `FileEntry.offset` note son
  décalage absolu en octets dans l'archive.

### `FileEntry` (postcard)

| Champ | Type | Signification |
| ----- | ---- | ------------- |
| `relative_path` | `String` | chemin relatif à la racine de l'archive |
| `offset` | `u64` | décalage absolu du flux dans l'archive |
| `permissions` | `Option<u32>` | permissions Unix à restaurer à l'extraction |
| `compression` | `CompressionType` | algorithme utilisé pour cette entrée |
| `file_nonce` | `[u8; 24]` | nonce principal propre à l'entrée |

### `MetaDatas` (postcard)

| Champ | Type | Signification |
| ----- | ---- | ------------- |
| `key_type` | `KeyType` | comment la clé doit être résolue |
| `compression` | `CompressionType` | algorithme de compression utilisé |
| `nonce` | `[u8; 24]` | nonce principal |

### `ArchiveHeader` (postcard)

| Champ | Type | Signification |
| ----- | ---- | ------------- |
| `magic` | `[u8; 4]` | `ENK1` |
| `version` | `u8` | la version d'Enkryptit! qui a écrit le fichier |
| `is_folder_archive` | `bool` | conteneur fichier vs. dossier |
| `meta_len` | `u32` | longueur des métadonnées de fin |

## Troncature & corruption

* Les flux de fichiers simples se terminent par `ENK1END` ; une fin de fichier inattendue
  correspond à `io::unexpected_eof`.
* Les archives de dossiers `version >= 2` conservent leurs métadonnées à la fin — un
  en-tête manquant est signalé comme `format::corrupted_file`.

## Liens

* [Erreurs](errors.md) — codes pour les artefacts corrompus/tronqués.
* [Installation](installation.md) — comment obtenir `eck`.
* Historique du format : `PROGRESSION.md` (jours 1 à 17).