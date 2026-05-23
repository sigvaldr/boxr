# boxr v2.3 - XZ Compression Edition (Parallel Multi-Threaded)

An archival utility written in Rust with maximum compression and multi-threaded parallel speed being the aim of the game.

---

## ⚠️ Compatibility Warning: Version Incompatibility

**boxr v2.x uses XZ compression, making it incompatible with boxr v1.x (which used Zstandard/zstd)!**

| Version | Compression Format | Archive Extension |
| ------- | ------------------ | ----------------- |
| v1.x    | Zstandard (zstd)   | `.tar.zst`        |
| v2.x    | XZ                 | `.tar.xz`         |

- **v1 archives (.box with .tar.zst inside) CANNOT be extracted by v2**
- **v2 archives (.box with .tar.xz inside) CANNOT be extracted by v1**

Migration path: To switch from v1 to v2, you must re-compress your folders using the new v2 tools. Old `.box` archives from v1 will not work with v2.

---

## Compression Format

This project uses **XZ compression** (.tar.xz inside .box).

XZ provides excellent compression ratios but is CPU-intensive. We've implemented:

### Streaming Compression (Memory Efficient)

- Uses streaming tar → XZ pipeline with ~4MB I/O buffer
- RAM usage reduced from O(directory_size) to ~5MB fixed buffer
- **~95% RAM reduction** during compression operations

### Multi-Threading (Parallel Speed)

- Up to **~5x speedup** on multi-core systems using all CPU cores
- liblzma automatically enables parallel compression with the appropriate library build
- Decompression also benefits from parallel processing

The underlying XZ algorithm is still CPU-bound and trades maximum speed for excellent compression ratios. See [XZ Compression](<https://en.wikipedia.org/wiki/XZ_(compression)>) for technical details.

---

## Usage

### Compressing a folder (boxr)

```bash
boxr <input_folder> [output_archive.box]
# Or with timestamp: boxr -stamp <input_folder> [output_archive.box]
```

### Extracting an archive (unboxr)

```bash
unboxr <archive.tar.xz> [-to <output_folder>]
```

---

## Building

```bash
cargo build --release
```

The resulting binaries work natively on each platform.

---

## Build Dependencies

### Prerequisites

- **Rust** 1.70+ and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

### System Dependencies for liblzma (XZ Compression)

The project uses `liblzma`, which requires the system's liblzma development libraries during build. At **runtime**, Windows systems need `liblzma.dll` available (same directory as `.exe` or in system PATH).

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

On Windows, use Rust's standard build tooling with MSVC toolchain. Install Visual Studio Build Tools with C++ Desktop Development workload, or use MinGW-w64 toolchain for GCC-based builds. The `liblzma` crate supports dynamic linking to system liblzma.

**Runtime Requirement:** Your compiled `.exe` will need `liblzma.dll` available at runtime. To get it:

- Download from [Tukaani XZ](https://tukaani.org/xz/) and place alongside your EXE
- Use Chocolatey (`choco install xz`) or Winget to install

#### Android (NDK)

Android requires NDK cross-compilation setup with appropriate target features enabled.

---

## License

MIT License
