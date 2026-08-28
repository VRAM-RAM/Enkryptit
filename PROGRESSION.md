# Progression map

This file report the progress since the 29/06/2026 (called as DAY-1) on `Enkryptit!`. Basic features have already been done. Note that work is not necessarily done on consecutive days. (DAY-1 can be on monday, but DAY-2 on friday of the same week).
The progress report will follow this format :

```md
---

## D-N - Key implementations / progress

Summary.

+ Implementation 1
+ Implementation 2
+ Implementation N

Possible known issues

---
```

---

## DAY-1 - Continued /test/ ; corrected `/src/compression.rs`

Most of the tests had `todo!()`. Fixed it by creating the tests. Also created `/src/lib.rs` in `eck` so that we can use the intern functions from another binary.

+ Created the compression unit tests
+ Created the encryption primitives unit tests
+ Created the metadatas unit tests
+ encryption flow tests are in progress, not yet finished.
+ Added a *retry* logic in `compression.rs` when buffer is too small

Current result on `cargo test --test eck_tests -- --nocapture` :

```bash
test result: FAILED. 77 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.78s
```

It is perfectly normal : folder tests (not implemented) fail. 

---

## DAY-2 - Added folder encryption / decryption

I added the folder encryption, and refactorized the code in `/src/encryption/`. Anyway, tests for folder encryption remains to be done.

* Created `/src/encryption/folder_encryption/`
* Added in this directory : `entries.rs` for collecting entries from a folder with `walkdir`, `inter_archive_encryption.rs` for encrypting / decrypting files from the archive.
* completed folder encryption / decryption 
* completed `object_treatment.rs` for supporting folder encryption

Test results :

```bash
test result: FAILED. 77 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.19s
```

It is normal, since I didn't already implemented the folder encryption tests.

---

## DAY-3 - Took over the project, added tests, and added comments

I added new tests (for folder encryption, finally + rustdoc), fixed existing tests and added doc comments.

* Modified almost every file to add doc comments.
* Modified `/test/integration/*`

Test results :

```bash
test result: ok. 89 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 8.21s
```

I also modified README.

---

## DAY-4 Fixed an issue with folder encryption

I fixed an issue with the folder encryption, in `/treatment/folder_case.rs`. 
\
The fact was that if you wanted to encrypt `path/to/my/folder/`, it created `path/to/my/folder/.encky`. 
\
It is not a huge issue, but I added a `strip_suffix()`, so that now it creates `path/to/my/folder.encky` correctly.

Test results :

```bash
test result: ok. 89 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 6.71s
```

---

## DAY-5 Refactorization & addon

I did a huge refactorization in the code. Now, the frontend doesn't resolves the `key`. 
\
It passes an `EnkryptitContext` (*mutable*) to the backend, so that the backend resolves both `Key` and `KeyType` easily :

```rust
let enkryptit_key = EnkryptitKey::resolve(Mode::Decrypting, &metadatas.key_type, context, path)?;
// Or 
let enkryptit_key = EnkryptitKey::resolve(Mode::Encrypting, &metadatas.key_type, context, path)?;
```

The `Key` and `KeyType` are now, in the backend, wrapped in `EnkryptitKey`, a more secure structure (`Zeroize` on drop, and calls `EnkryptitContext::resolve` wich contains the password as `Zeroizing<>`. Also, the key is *Locked* (`mlock`)).
\
Because of these refactorizations, the code is now easier to read (and to maintain), but also allows the user to do :
```bash
eck myfile.txt myfile2.txt
eck myfolder/*
```

> Many files can be encrypted with one command now.

This is possible because of `pub fn treat_objects_with_multiple_paths()` and the modification of the Cli's arguments (`Vec<String>` for paths instead of `Option<String>`.)
\
Files added :
- frontend/params_helper.rs
\
frontend/mod.rs
\
frontend/treat_output.rs
\
frontend/treatment.rs
\
frontend/tui.rs (moved)
\
frontend/cli.rs (moved)

- context.rs
- key/derivation.rs
\
key/generation.rs
\
key/mod.rs
\
key/resolve.rs
\
key/storage.rs

- Files modified : Almost all, except `compression`, `metadatas`, `parameters` and `encryption_primitives`.

Test results :

```bash
test result: ok. 88 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.63s
```

(But many tests are deprecated)

---

## DAY-6 Ergonomic addon 

Added a `Browse` section in the TUI that uses `rfd` (Rusty Files Dialogs), and allows the users to now browse their **files** and **folders**.

- Files modified : frontend/tui.rs
- Functions added :
    - pub fn launch_browser() -> Result<(), EnkryptitError> {} (launches the browser menu)
    - fn browse_files() -> Result<(), EnkryptitError> {} (opens the `Browse Files` pop-up, and treats the selected objects)
    - fn browse_folders() -> Result<(), EnkryptitError> {} (opens the `Browse Folders` pop-up, and treats the selected objects)
    - fn browse_files_then_folders() -> Result<(), EnkryptitError> {} (opens the `Browse Files` pop-up, then the `Browse Files` pop-up, combines the two results, and treats the selected objects)

Test results :

```bash
test result: ok. 88 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 5.21s
```

But **too many** tests are deprecated, and I need to add test for :
- testing `eck /myfile.rs /myfolder/ /myfolder2/*`
- tests for the `EnkryptitContext` (password resolution, essentially)
- tests for `EnkryptitKey`
- tests for the `Browse` section

---