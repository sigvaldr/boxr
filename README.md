# boxr v2.0 - XZ Compression Edition

An archival utility written in Rust with maximum compression being the aim of the game.

---

## ⚠️ Compatibility Warning: Version Incompatibility

**boxr v2.0 uses XZ compression, making it incompatible with boxr v1.0!**

| Version | Compression Format | Archive Extension |
| ------- | ------------------ | ----------------- |
| v1.x    | Zstandard (zstd)   | `.tar.zst`        |
| v2.x    | XZ                 | `.tar.xz`         |

- **v1 archives (.box with .tar.zst inside) CANNOT be extracted by v2**
- **v2 archives (.box with .tar.xz inside) CANNOT be extracted by v1**

Migration path: To switch from v1 to v2, you must re-compress your folders using the new v2 tools. Old `.box` archives from v1 will not work with v2.

---

## Compression Format

This project now uses **xz compression** (.tar.xz) instead of zstd (.tar.zst).

XZ provides better compression ratios but slower compression speeds compared to Zstandard.

## Usage

### Compressing a folder (boxr)

```bash
boxr <input_folder> [output_archive.box]
```

Or with timestamp:

```bash
boxr -stamp <input_folder> [output_archive.box]
```

### Extracting an archive (unboxr)

```bash
unboxr <archive.tar.xz> [-to <output_folder>]
```

## Building

```bash
cargo build --release
```

## Requirements

- Rust 1.70+ and Cargo installed
- `xz` development libraries for xz compression (usually pre-installed on Linux)

## License

MIT License
