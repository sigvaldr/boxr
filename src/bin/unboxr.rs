// ------------------------------------------------------------------------------
// Author        : Sigvaldr Nótthrafn
// Project       : boxr
// File          : unboxr.rs
// Creation Date : 08JUL2025
// ------------------------------------------------------------------------------

use std::env;
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::{Path, PathBuf};
use std::process;

use liblzma::read::XzDecoder;
use tar::Archive;
use zstd::stream::read::Decoder;

const VERSION: &str = "2.3.0";

// XZ magic number (little-endian) - first 4 bytes of XZ frame
const XZ_MAGIC: u32 = 0xFD7A77BF;

// ZSTD magic number (little-endian) - first 4 bytes of ZSTD frame header
const ZSTD_MAGIC: u32 = 0xFD2FB528;

/// Read and parse first 4 bytes to return the magic number as little-endian u32
fn read_magic(bytes: &[u8]) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// Check if a magic number indicates XZ format
fn is_xz_magic(magic: u32) -> bool {
    // XZ magic numbers range from 0xFD7A77BF to 0xFD7A77FF (first byte must be 0xFD)
    magic >= 0xFD7A77BF && magic <= 0xFD7A77FF
}

/// Extract archive - auto-detects XZ or ZSTD compression format
fn extract_archive(
    archive_path: &str,
    output_folder: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let oname = output_folder.to_string_lossy().to_string();
    println!("Extracting {} into {}", archive_path, oname);

    // Open the archive file and read first 4 bytes (magic number)
    let mut file_handle = File::open(archive_path)?;
    let mut magic_buffer = [0u8; 4];
    std::io::Read::read_exact(&mut file_handle, &mut magic_buffer)?;

    // Parse magic number and seek back to start
    let magic = read_magic(&magic_buffer)?;
    file_handle.seek(std::io::SeekFrom::Start(0))?;

    match magic {
        m if is_xz_magic(m) => {
            println!("Detected XZ compression");
            std::fs::create_dir_all(output_folder)?;
            let file = BufReader::new(file_handle);
            Archive::new(XzDecoder::new(file)).unpack(output_folder)?;
        }
        m if m == ZSTD_MAGIC => {
            println!("Detected ZSTD compression");
            std::fs::create_dir_all(output_folder)?;
            let file = BufReader::new(file_handle);
            let decoder =
                Decoder::new(file).map_err(|e| format!("ZSTD decompression error: {}", e))?;
            Archive::new(decoder).unpack(output_folder)?;
        }
        magic => {
            return Err(format!(
                "Unsupported compression format. Expected XZ (0x{:08X}) or ZSTD (0x{:08X}), got 0x{:08X}",
                XZ_MAGIC, ZSTD_MAGIC, magic
            ).into());
        }
    };

    Ok(())
}

fn main() {
    println!("unBoxr v{} by Sigvaldr", VERSION);
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <archive.tar.xz|tar.zst> [-to <output_folder>]",
            args[0]
        );
        process::exit(1);
    }

    let archive_path = &args[1];
    let output_folder = parse_output_folder(&args);

    if let Err(e) = extract_archive(archive_path, &output_folder) {
        eprintln!("Extraction failed: {}", e);
        process::exit(1);
    }

    println!("Archive extracted to '{}'", output_folder.display());
}

fn parse_output_folder(args: &[String]) -> PathBuf {
    if let Some(pos) = args.iter().position(|arg| arg == "-to") {
        if let Some(folder) = args.get(pos + 1) {
            return PathBuf::from(folder);
        } else {
            eprintln!("Error: -to flag requires a folder name");
            process::exit(1);
        }
    }

    // Default: use archive name without `.tar.xz` or `.tar.zst`
    let archive_name = Path::new(&args[1]);
    let stem = archive_name
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    // Handle double extensions like `.tar.xz` or `.tar.zst`
    let folder_name = if stem.ends_with(".tar") {
        stem.strip_suffix(".tar").unwrap_or(&stem)
    } else {
        &stem
    };

    PathBuf::from(folder_name)
}
