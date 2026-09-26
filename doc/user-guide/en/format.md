# The `.encky` format 

An encrypted artifact is written as **`<stem>.encky`**. There are **two** container kinds:

1. a **single-file** encrypted file,
2. a **folder archive** (a whole directory in one `.encky`).

Everything is serialized with **postcard**; the version is stored in the header
(`VERSION` constant, currently `3` — the 3rd nightly of the first stable line).

## Common primitives

* Magic number: `0x45 0x4E 0x4B 0x31` = `ENK1`.
* Chunk size: `CHUNK_SIZE = 8 MiB`.
* Cipher: **XChaCha20-Poly1305** (24-byte nonce, AEAD).
* A 32-byte key (from password / key file / OS keyring).

## Master nonce & derived nonces

Each object (or archive entry) has a **master nonce** stored in its metadata. For every
chunk `step`, a derived nonce is built:

```
new_nonce[0..16]  = master_nonce[0..16]
new_nonce[16..24] = step.to_le_bytes()   # 8 bytes, little-endian
```

## Encrypted stream (payload)

For a single logical stream the payload is a sequence of chunks followed by an end marker:

```
[ chunk_len:u32 LE ][ compressed_fragment ]
[ chunk_len:u32 LE ][ compressed_fragment ]
...
[ "ENK1END" ]        # 7 literal bytes
```

* Each chunk is **compressed**, then **encrypted** with the derived nonce of its step.
* The compressed part is placed in a buffer up to `CHUNK_SIZE`, but each written fragment
  is preceded by its actual length (`u32`, little-endian).

## Single-file layout

```
offset 0 : header_len:u8                       # postcard header length (>= 1 byte)
          ArchiveHeader := magic[4] + version:u8 + is_folder_archive:false + meta_len:u32
          MetaDatas     := key_type + compression + nonce[24]
          then the encrypted payload stream (see above), ended by ENK1END
```

`payload_offset` for decryption = `1 + header_len + meta_len`.

## Folder archive layout (v2+)

```
byte 0                         : HEADER_REGION_SIZE:u8 (= 64)
bytes 1 .. 65                  : ArchiveHeader, zero-padded to 64 bytes
                                 (magic, version, is_folder_archive:true, meta_len)
bytes 65 ..                    : one encrypted stream per file, sequentially,
                                 each with its own file_nonce (see FileEntry)
end of file                    : FolderMetadata (postcard)
                                 = key_type + entries[ FileEntry ]
```

* `HEADER_REGION_SIZE` keeps the header area fixed so the header can be patched in place
  without changing the region size.
* Entries are encrypted one after the other; each `FileEntry.offset` records its absolute
  byte offset inside the archive.

### `FileEntry` (postcard)

| Field | Type | Meaning |
| ----- | ---- | ------- |
| `relative_path` | `String` | path relative to the archive root |
| `offset` | `u64` | absolute offset of the stream in the archive |
| `permissions` | `Option<u32>` | Unix permissions to restore on extract |
| `compression` | `CompressionType` | algorithm used for that entry |
| `file_nonce` | `[u8; 24]` | per-entry master nonce |

### `MetaDatas` (postcard)

| Field | Type | Meaning |
| ----- | ---- | ------- |
| `key_type` | `KeyType` | how the key must be resolved |
| `compression` | `CompressionType` | algorithm used |
| `nonce` | `[u8; 24]` | master nonce |

### `ArchiveHeader` (postcard)

| Field | Type | Meaning |
| ----- | ---- | ------- |
| `magic` | `[u8; 4]` | `ENK1` |
| `version` | `u8` | the Enkryptit! version that wrote the file |
| `is_folder_archive` | `bool` | file vs. folder container |
| `meta_len` | `u32` | length of the trailing metadata |

## Truncation & corruption

* Single-file streams end with `ENK1END`; an unexpected EOF maps to `io::unexpected_eof`.
* Folder archives with `version >= 2` keep their metadata at the end — a missing trailer is
  reported as `format::corrupted_file`.

## Related

* [Errors](errors.md) — codes for corrupted/truncated artifacts.
* [Installation](installation.md) — how to get `eck`.
* Format history: `PROGRESSION.md` (days 1–17).