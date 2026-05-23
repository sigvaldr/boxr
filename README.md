# 📦 Boxr v2.4 - XZ & ZSTD Dual-Format Edition

A high-performance archival utility written in **Rust** for maximum compression and multi-threaded parallel speed. Supports both XZ and ZSTD compression formats with automatic format detection.

---

## ✨ Features

- **Dual Format Support**: Automatically handles both `.tar.xz` (XZ) and `.tar.zst` (ZSTD) archives
- **Maximum Compression**: XZ compression with liblzma for excellent compression ratios
- **Multi-Threading**: Parallel compression/decompression using all available CPU cores (~5x speedup on multi-core systems)
- **Memory Efficient**: Streaming pipeline uses only ~4MB I/O buffer (~95% RAM reduction vs full materialization)

---

## 📦 Archive Format Support

| Version | Compression Format | Extension (XZ) | Extension (ZSTD) |
| ------- | ------------------ | -------------- | ---------------- |
| v1.x    | Zstandard (zstd)   | —              | `.tar.zst`       |
| v2.4+   | **XZ + ZSTD**      | `.tar.xz`      | `.tar.zst`       |

> ℹ️ **Auto-Detection**: The `unboxr` tool automatically detects the compression format by reading magic bytes. It prefers ZSTD if detected, otherwise assumes XZ.

---

## 🛠️ Usage

### Compressing a folder (`boxr`)

```bash
# Basic compression (auto output as .box)
boxr <input_folder>

# With custom output filename
boxr <input_folder> output.box

# Add timestamp to filename (format: DDMMMYY-HHMMSS-STEM.box)
boxr -stamp <input_folder>

# Explicit output path
boxr <input_folder> /path/to/archive.box
```

### Extracting an archive (`unboxr`)

```bash
unboxr <archive.box> [-to <output_folder>]

unboxr /path/to/archive.box
unboxr archive.box -to /path/to/output
```

---

## 📋 Quick Reference

| Command                         | Description                          |
| ------------------------------- | ------------------------------------ |
| `boxr <folder>`                 | Compress folder to auto `.box`       |
| `boxr -stamp <folder>`          | Compress with date stamp in filename |
| `boxr <folder> output.box`      | Compress with custom filename        |
| `unboxr archive.box [-to dest]` | Extract archive                      |

---

## 🦀 Building

```bash
# Debug build (includes compression detection messages in output)
cargo build --bin boxr           # Development with debug assertions
cargo build --bin unboxr         # Development with debug assertions

# Release build (optimized, faster, no debug messages)
cargo build --release --bin boxr
cargo build --release --bin unboxr

# Install to system path
cargo install --path . --bin boxr --bin unboxr
```

**To see compression format detection output**, run with the debug build. Release builds operate silently for optimal performance.

---

## 🔧 Build Dependencies

### Prerequisites

- **Rust** 1.70+ and Cargo installed
- [Install Rust](https://www.rust-lang.org/tools/install)

### System Dependencies for liblzma (XZ Compression)

The project uses `liblzma` which requires system development libraries during build. At runtime, Windows systems need `liblzma.dll` available (same directory as `.exe` or in system PATH).

#### Linux (Debian/Ubuntu)

```bash
sudo apt-get install liblzma-dev
# or for older distros: sudo yum install xz-devel
```

#### Linux (Fedora/RHEL/CentOS)

```bash
sudo dnf install xz-devel
# or: sudo yum install xz-devel
```

#### macOS (Homebrew)

```bash
brew install xz
```

#### Windows (MSVC/MinGW)

- Use Rust's standard build tooling with **MSVC** toolchain (Visual Studio Build Tools + C++ Desktop Development workload) or **MinGW-w64** for GCC-based builds.

> ⚠️ **Runtime Requirement**: On Windows, `liblzma.dll` must be available at runtime. Download from [Tukaani XZ](https://tukaani.org/xz/) and place alongside your EXE, or use Chocolatey (`choco install xz`) / Winget to install.

---

## 📊 Performance Notes

- **Compression**: Uses streaming tar → XZ pipeline with multi-threading enabled via liblzma
- **Decompression**: Supports both XZ (liblzma) and ZSTD (libzstd) decoders automatically
- **Memory Usage**: Fixed ~4MB I/O buffer regardless of archive size

---

## 🔒 License

MIT License
