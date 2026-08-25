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