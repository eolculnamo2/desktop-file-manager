use std::{fs, io, path::PathBuf};

use crate::{app_entry::AppEntry, generate_entry_file_contents::generate_map_contents_from_entry};

// current goal is to replace same (relative) line number
// with updates and leave other lines untouched
/// Syncs file system with updated entry
pub fn update_entry(entry: AppEntry) -> Result<(), io::Error> {
    let lines_to_replace_map = generate_map_contents_from_entry(&entry);
    let file_bytes = fs::read(&entry.absolute_path)?;
    let file_string = String::from_utf8_lossy(&file_bytes);

    let new_file_str: String = file_string
        .lines()
        .map(|l| {
            let mut split = l.split("=");
            if let Some(key) = split.next() {
                if let Some(replacement_line) = lines_to_replace_map.get(key) {
                    return format!("{}\n", replacement_line);
                }
            }
            format!("{}\n", l)
        })
        .collect();

    // need to test this will before replacing it  with live files
    let test_file = PathBuf::from("/home/rob/test.desktop");
    fs::write(test_file, new_file_str)?;
    // fs::write(entry.absolute_path, new_file_str)?;
    Ok(())
}

// pub fn create_entry() {
// generate_header()
// }