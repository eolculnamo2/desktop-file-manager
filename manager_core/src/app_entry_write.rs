use std::{fs, io};

use crate::{
    app_entry::AppEntry,
    generate_entry_file_contents::{
        generate_map_contents_from_entry, generate_vec_contents_from_entry,
    },
};

pub fn create_entry(entry: AppEntry) -> Result<(), io::Error> {
    let file_text_join = generate_vec_contents_from_entry(&entry).join("");
    let file_text = file_text_join.as_str();
    fs::write(entry.absolute_path, file_text)?;
    Ok(())
}

// current goal is to replace same (relative) line number
// with updates and leave other lines untouched
/// Syncs file system with updated entry
pub fn update_entry(entry: AppEntry) -> Result<(), io::Error> {
    let mut lines_to_replace_map = generate_map_contents_from_entry(&entry);
    let file_bytes = fs::read(&entry.absolute_path)?;
    let file_string = String::from_utf8_lossy(&file_bytes);

    let new_file_str: String = file_string
        .lines()
        .map(|l| {
            let mut split = l.split("=");
            if let Some(key) = split.next() {
                // get and remove line if it exists to replace old entries with new
                if let Some(replacement_line) = lines_to_replace_map.remove(key) {
                    return format!("{}\n", replacement_line);
                }
            }
            format!("{}\n", l)
        })
        .collect();

    // handle entries that original file does not contain if they exist
    // This could cause bugs if it appends to line that doensn't have new line at end.. Should
    // really write unit tests
    let new_entries = lines_to_replace_map
        .values()
        .fold(String::new(), |agg, acc| {
            format!(
                "{}{}",
                agg,
                // if agg.ends_with('\n') { "" } else { "" },
                acc
            )
        });

    // need to test this will before replacing it  with live files
    // let test_file = PathBuf::from("/home/rob/test.desktop");
    // fs::write(test_file, new_file_str)?;
    fs::write(
        entry.absolute_path,
        format!("{}\n{}", new_file_str, new_entries),
    )?;
    Ok(())
}

// pub fn create_entry() {
// generate_header()
// }
