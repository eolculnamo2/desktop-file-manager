use std::collections::HashMap;

use crate::app_entry::AppEntry;

pub fn generate_header() -> String {
    String::from("[Desktop Entry]\nVersion=1.1\n")
}

// this is fine for new, but we need to preserve the rest
// of the lines in the file for update

/// Generates vector of line strings to for app
pub fn generate_vec_contents_from_entry(entry: &AppEntry) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let app_type_str: String = entry.app_type.clone().into();
    content.push(format!("Type={}\n", app_type_str));
    content.push(format!("Name={}\n", entry.name));
    content.push(format!("Generic Name={}\n", entry.name));
    if let Some(comment) = &entry.comment {
        content.push(format!("Comment={}\n", comment));
    }
    if let Some(icon) = &entry.icon {
        content.push(format!("Icon={}\n", icon));
    }
    content.push(format!("Exec={}\n", entry.exec));
    content.push(format!("Categries={}\n", entry.categories.join(",")));
    content
}

/// Generates vector of line strings to for app
pub fn generate_map_contents_from_entry(entry: &AppEntry) -> HashMap<&str, String> {
    let mut content: HashMap<&str, String> = HashMap::new();
    let app_type_str: String = entry.app_type.clone().into();
    content.insert("Type", format!("Type={}", app_type_str));
    content.insert("Name", format!("Name={}", entry.name.clone()));
    // content.insert("Generic Name", entry.name);
    if let Some(comment) = &entry.comment {
        content.insert("Comment", format!("Comment={}", comment));
    }
    if let Some(icon) = &entry.icon {
        content.insert("Icon", format!("Icon={}", icon));
    }
    content.insert("Exec", format!("Exec={}", entry.exec));
    if entry.categories.len() > 0 {
        content.insert(
            "Categories",
            format!("Categories={};", &entry.categories.join(";").as_str()),
        );
    }
    content
}
