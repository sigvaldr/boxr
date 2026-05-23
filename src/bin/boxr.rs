// ------------------------------------------------------------------------------
// Author        : Sigvaldr Nótthrafn
// Project       : boxr
// File          : boxr.rs
// Creation Date : 08JUL2025
// ------------------------------------------------------------------------------

use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use tar::Builder;
use xz2::write::XzEncoder;

const VERSION: &str = "2.0.0";

fn main() {
    println!("Boxr v{} by Sigvaldr", VERSION);
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
    let parent = original.parent().unwrap_or_else(|| Path::new(""));
    let new_name = format!("{date}-{stem}.box");

    parent.join(new_name)
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

    // Create a temporary file for the tar.xz archive
    let temp_path = output_file.with_extension("tar.xz");

    // Build the tar archive in memory first
    let mut tar_data: Vec<u8> = Vec::with_capacity(4 * 1024 * 1024);
    {
        let mut builder = Builder::new(std::io::Cursor::new(&mut tar_data));
        builder.append_dir_all(".", input_folder)?;
    }

    // Write the tar data through xz encoder directly to file
    let tar_file = File::create(&temp_path)?;
    let mut encoder = XzEncoder::new(tar_file, 9);
    encoder.write_all(&tar_data)?;
    encoder.finish()?;

    // Rename to .box extension
    fs::rename(temp_path, output_file)?;

    println!("Done.");
    Ok(())
}
