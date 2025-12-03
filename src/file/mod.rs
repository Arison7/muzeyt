use std::fs;
use std::io;

pub fn read_files(folder: &str) -> io::Result<Vec<String>> {
    let files = fs::read_dir(folder)?
        .filter_map(|entry| {
            let entry = entry.ok()?; // Skip unreadable entries
            let path = entry.path();
            if path.is_file() {
                path.file_name().map(|n| n.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(files)
}
