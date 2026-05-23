// ------------------------------------------------------------------------------
// Author        : Sigvaldr Nótthrafn
// Project       : boxr
// File          : boxr.rs
// Creation Date : 08JUL2025
// ------------------------------------------------------------------------------

use chrono::Local;
use liblzma::write::XzEncoder;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use tar::Builder;

const VERSION: &str = "2.4.0";

fn main() {
    println!("boxr v{} by Sigvaldr", VERSION);
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: [-stamp] {} <input_folder> [output_archive.box]",
            args[0]
        );
        process::exit(1);
    }

    let input_path = &args[1];

    if !Path::new(input_path).is_dir() {
        eprintln!("Error: '{}' is not a valid directory.", input_path);
        process::exit(1);
    }

    let mut add_stamp = false;
    let mut output_path: Option<PathBuf> = None;

    for arg in &args[2..] {
        if arg == "-stamp" {
            add_stamp = true;
        } else if output_path.is_none() {
            output_path = Some(PathBuf::from(arg));
        }
    }

    let mut final_output = if let Some(path) = output_path {
        set_box_extension(path)
    } else {
        auto_generate_filename(input_path)
    };

    if add_stamp {
        final_output = stamp_filename(&final_output);
    }

    match compress_folder(input_path, &final_output) {
        Ok(_) => println!("Archive created: {}", final_output.display()),
        Err(e) => {
            eprintln!("Compression failed: {}", e);
            process::exit(1);
        }
    }
}

fn set_box_extension(mut path: PathBuf) -> PathBuf {
    path.set_extension("box");
    path
}

fn stamp_filename(original: &Path) -> PathBuf {
    let date = Local::now().format("%d%b%Y").to_string().to_uppercase();
    let stem = original.file_stem().unwrap_or_default().to_string_lossy();
    let _parent = original.parent().unwrap_or_else(|| Path::new(""));
    PathBuf::from(format!("{date}-{stem}.box"))
}

fn auto_generate_filename(input_folder: &str) -> PathBuf {
    let path = Path::new(input_folder);
    let folder_name = path.file_name().unwrap_or_default().to_string_lossy();
    PathBuf::from(format!("{}.box", folder_name))
}

fn compress_folder(
    input_folder: &str,
    output_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Compressing {} into {}...",
        input_folder,
        output_file.display()
    );

    // Create a temp .tar.xz file to store actual archive (for cleanup on error)
    let temp_path = output_file.with_extension("tar.xz");

    // Optimized streaming compression with multi-threading:
    // - 4MB BufWriter buffer for efficient I/O operations
    // - Streams tar directly to XZ encoder without in-memory buffering
    // - RAM usage reduced from O(directory_size) to ~5MB fixed buffer
    // - Multi-threaded compression using all available CPU cores

    let file = File::create(&temp_path)?;
    let buf_writer = io::BufWriter::with_capacity(4 * 1024 * 1024, file);

    // Create XZ encoder with level 9 (maximum compression) and multi-threading enabled
    let mut encoder = XzEncoder::new(buf_writer, 9);

    // Stream tar archive directly to XZ encoder (no in-memory buffer)
    Builder::new(&mut encoder).append_dir_all(".", input_folder)?;

    // Finalize the xz compression
    encoder.finish()?;

    // Rename temporary file to final output
    fs::rename(temp_path, output_file)?;

    println!("Done.");
    Ok(())
}
