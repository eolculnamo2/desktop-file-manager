use std::{fs, io, path::Path};

use crate::{
    app_entry::AppEntry,
    constants::location_constants::{get_shared_app, get_user_app, SHARED_TYPE, USER_TYPE},
    generate_entry_file_contents::{
        generate_header, generate_map_contents_from_entry, generate_vec_contents_from_entry,
    },
    logger::Log,
};

#[derive(Debug, Clone)]
pub enum CreateEntryError {
    InvalidPathType,
    MissingPathType,
    FileAlreadyExists,
    UnableToVerifyFileAvailable,
    IoError,
}

impl From<io::Error> for CreateEntryError {
    fn from(_: io::Error) -> Self {
        CreateEntryError::IoError
    }
}

fn convert_name_to_desktop_name(entry_name: &str) -> String {
    let sanitized_name = entry_name.replace(" ", "_").replace('\n', "");
    if sanitized_name.ends_with(".desktop") {
        sanitized_name
    } else {
        format!("{}.desktop", sanitized_name)
    }
}

pub fn create_entry(entry: AppEntry) -> Result<(), CreateEntryError> {
    let file_text_join = generate_vec_contents_from_entry(&entry).join("");
    let file_text_raw = file_text_join.as_str();
    let file_header = generate_header();
    let file_text = format!("{}\n{}", file_header, file_text_raw);

    // taking a shortcut and making absolute path be hard coded local or shared for now
    // will clean up later
    let path_type = entry
        .absolute_path
        .to_str()
        .ok_or(CreateEntryError::MissingPathType)?;

    let user_path = get_user_app();
    let absolute_path = if path_type == SHARED_TYPE {
        Ok(get_shared_app())
    } else if path_type == USER_TYPE {
        Ok(user_path.to_str().expect("Unable to get user app path"))
    } else {
        Log::error(format!("Invalid file type on save [[{}]]", path_type)).send_log();
        Err(CreateEntryError::InvalidPathType)
    }?;

    let file_name = convert_name_to_desktop_name(&entry.name);
    let path_with_file_name = format!("{}/{}", absolute_path, file_name);

    let already_exists_result = Path::try_exists(Path::new(&path_with_file_name));
    if let Err(e) = already_exists_result {
        let error_msg = format!("Unable to determine if file aleady exists: [[{:?}]]", e);
        Log::error(error_msg).send_log();
        return Err(CreateEntryError::UnableToVerifyFileAvailable);
    } else if let Ok(true) = already_exists_result {
        Log::error(format!("File [[{}]] already exists", path_with_file_name)).send_log();
        return Err(CreateEntryError::FileAlreadyExists);
    }
    fs::write(path_with_file_name, file_text)?;
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
