
# mmap_bidiff

`mmap_bidiff` is a binary diffing and patching system designed for **zero-copy** and **memory-mapped** operation on large files. It provides a way to compute binary differences and apply patches without allocating or copying large buffers.

This repository is a merge of three existing crates: `files-diff`, `bidiff` and `bipatch`, whose systems have undergone a deep rewrite to support **zero-copy memory-mapped diffing and patching**.

---

## Added Features

- ✅ **Zero-copy** diff and patch operations via `memmap2`
- ✅ Supports large files via streaming, mmap-backed processing

---

## Patch Format

Patches consist of:
- A metadata header (version, algorithm, compression, hashes)
- A stream of `Add` and `Copy` instructions encoded using varints
- Optional compressed payload (configurable)

All operations are:
- **Streaming-safe**
- **Validated with hashes**
- **Exact-size encoded**

---

## Codebase Change Summary

### ✅ Substantial Changes (Rewritten or Deeply Modified)

#### `bidiff`
- Instruction format preserved, but encoding logic fully refactored.
- Streaming diff logic rewritten to support direct mmap output.
- Patch metadata header added for format introspection and validation.
- Enforced patch size guarantees with deterministic output layout.

#### `bipatch`
- Patch reading refactored into a `ZeroCopyReader` with explicit cursor advancement.
- Fully zero-copy patch application with mmap input/output.
- Allocation-based logic removed entirely.
- Error handling and validation integrated for safety.

#### `files-diff` (consumer layer)
- Completely rewritten to orchestrate mmap-backed diffing and patching.
- Replaces all in-memory `Cursor` usage with `MmapReader` / `MmapWriter`.
- Integrates temporary file handling, patch validation, and compression.
- Ensures deterministic, safe disk-backed patching in real-world workflows.
- **All support for ZIP file input/output removed**.
- **All support for `rsync20` algorithm removed** – only `bidiff1` remains.

---

### ⚠️ Moderately Changed

- Compression logic and metadata serialization upgraded and standardized.
- Custom error types, result handling, and validation logic added.
- Updated end-to-end test using tempfiles and disk-backed verification.

---

### ✅ Preserved / Compatible

- **Patch format**: Stream of `add` / `copy` instructions with varint encoding.
- **Conceptual I/O model**: `before → patch → after`, using `diff()` and `apply()`.
- **Compression model**: Still pluggable (e.g., Zstd).

---

### 🧮 Summary

| Category             | Status       |
|----------------------|--------------|
| Diff Logic           | 🔄 Rewritten |
| Patch Application    | 🔄 Rewritten |
| Memory Model         | 🔄 From buffer-based → mmap zero-copy |
| Error Handling       | 🔄 Enhanced |
| Test Infrastructure  | 🔄 Rewritten with tempfiles |
| Instruction Format   | ✅ Preserved |
| Compression          | ✅ Compatible |
| Output Determinism   | ✅ Enforced |
| ZIP File Handling    | ❌ Removed |
| rsync20 Algorithm    | ❌ Removed |
| API Compatibility    | ⚠️ Conceptual match, impl diverges |
| Estimated Code Changed | **~75–85%** |

This codebase now supports **patching with zero-copy, mmap-safe, deterministic performance**, suitable for large binary files.

---

## Usage

### Diffing

```rust
use mmap_bidiff::{self, CompressAlgorithm};
use std::path::Path;

let metadata = mmap_bidiff::diff(
    Path::new("before.bin"), // <-- input 1
    Path::new("after.bin"),  // <-- input 2
    Path::new("patch.bin"),  // <-- output
    CompressAlgorithm::Zstd,
)?;
```

### Applying

```rust
use std::fs::Path;

mmap_bidiff::apply(
    Path::new("before.bin"), // <-- input 1
    Path::new("patch.bin"),  // <-- input 2
    Path::new("after.bin"),  // <-- output
)?;
```

## Architecture
- bidiff/ – Instruction encoder + core diff algorithm (bidiff1)
- bipatch/ – Streaming patch parser and applier (zero-copy)
- files-diff/ – Top-level orchestration layer (hashing, compression, I/O)

## Status

Still in testing.
Soon to be prototype-ready for:
- Backup systems
- Sync engines
- Binary versioning tools
- Embedded update frameworks

## Licensing

This project includes code derived from:

- `bidiff` (MIT OR Apache-2.0 license)
- `files-diff` (BSD 2-Clause License)

All licenses and attributions are preserved in the LICENSE file.

The codebase has been substantially rewritten for:
- Zero-copy memory-mapped patching
- Streaming-optimized diff and apply

All modified code is provided under the MIT license. Attribution and license text preserved per original terms.
