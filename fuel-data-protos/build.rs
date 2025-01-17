use std::fs;
use std::path::Path;

use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_bin_path = protoc_bin_vendored::protoc_bin_path()?;
    std::env::set_var("PROTOC", protoc_bin_path);

    let protos_root = "protos"; // Root directory containing your .proto files

    // Collect all .proto files in the directory and its subdirectories
    let proto_files: Vec<_> = WalkDir::new(protos_root)
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore errors
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "proto")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();

    let output_dir = &Path::new("generated/rust");

    // Compile the collected .proto files with tonic-build
    tonic_build::configure()
        .out_dir(output_dir)
        .compile_protos(&proto_files, &[protos_root])?;

    // Replace "super::super" or deeper chains with "crate"
    replace_two_or_more_supers_with_crate_in_dir(output_dir)?;

    Ok(())
}

fn replace_two_or_more_supers_with_crate_in_dir(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively process subdirectories
            replace_two_or_more_supers_with_crate_in_dir(&path)?;
        } else if let Some(extension) = path.extension() {
            if extension == "rs" {
                // Process only Rust files
                let content = fs::read_to_string(&path)?;
                // Replace "super::super" or more with "crate"
                let updated_content = replace_two_or_more_supers_with_crate(&content);
                fs::write(&path, updated_content)?;
                println!("Updated: {:?}", path);
            }
        }
    }
    Ok(())
}

fn replace_two_or_more_supers_with_crate(content: &str) -> String {
    // Replace "super::super::" or deeper chains with "crate"
    let pattern = r"(super::){2,}";
    let regex = regex::Regex::new(pattern).unwrap();
    regex.replace_all(content, "crate::").to_string()
}
