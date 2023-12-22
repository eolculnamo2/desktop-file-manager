use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app_entry::{AppEntry, AppType, EntryTypes};

fn parse_categories(raw: &str) -> Vec<String> {
    let mut categories: Vec<String> = vec![];
    let mut split = raw.split(";");
    while let Some(value) = split.next() {
        if !value.is_empty() {
            categories.push(value.to_string());
        }
    }
    categories
}

fn get_property_from_text_line(line: &str) -> Option<EntryTypes> {
    let mut split = line.split("=");
    if let Some(kind) = split.next() {
        if let Some(value) = split.next() {
            return Some(match kind {
                "Type" => EntryTypes::AppTypeEntry(AppType::from(value.to_string())),
                // TODO handle generic name better
                "Name" => EntryTypes::Name(value.to_string()),
                "Comment" => EntryTypes::Comment(value.to_string()),
                "Icon" => EntryTypes::Icon(value.to_string()),
                "Exec" => EntryTypes::Exec(value.to_string()),
                "Terminal" => EntryTypes::Terminal(value.to_lowercase() == "true"),
                "Categories" => EntryTypes::Categories(parse_categories(value)),
                _ => EntryTypes::Ignore,
            });
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesktopFileParseError {
    InvalidHeader,
    ReadError,
}

impl From<std::io::Error> for DesktopFileParseError {
    // this isn't ideal because we swallow the IO error
    // but we cant implement PartialEq or Eq with it
    fn from(_: std::io::Error) -> Self {
        DesktopFileParseError::ReadError
    }
}

#[cfg(test)]
const SAMPLE: &'static [u8]  = b"[Desktop Entry]\nVersion=1.1\nType=Application\nName=Firefox\nGenericName=Firefox\nComment=A small descriptive blurb about this application.\nIcon=/Apps/firefox/browser/chrome/icons/default/default48.png\nExec=/Apps/firefox/firefox\nActions=\nCategories=WebBrowser;X-GNOME-Other;X-GNOME-WebApplications;\n";

pub fn get_entry_from_file(location: PathBuf) -> Result<AppEntry, DesktopFileParseError> {
    #[cfg(not(test))]
    let file = std::fs::read(location.clone())?;

    #[cfg(test)]
    let file = SAMPLE;

    let text = String::from_utf8_lossy(&file);

    if text.starts_with("[Desktop Entry]") == false {
        eprintln!("invalid text {:?}", text);
        return Err(DesktopFileParseError::InvalidHeader);
    }

    let parts = text.split('\n');
    let mut app_entry = AppEntry::new(location);
    for part in parts {
        if let Some(value) = get_property_from_text_line(part) {
            match value {
                EntryTypes::AppTypeEntry(entry) => app_entry.app_type = entry,
                EntryTypes::EncodingEntry(entry) => app_entry.encoding = entry,
                EntryTypes::Name(entry) => app_entry.name = entry,
                EntryTypes::Comment(entry) => app_entry.comment = Some(entry),
                EntryTypes::Icon(entry) => app_entry.icon = Some(entry),
                EntryTypes::Exec(entry) => app_entry.exec = entry,
                EntryTypes::Terminal(entry) => app_entry.terminal = entry,
                EntryTypes::Categories(entry) => app_entry.categories = entry,
                EntryTypes::Ignore => (),
            }
        }
    }
    Ok(app_entry)
}

#[cfg(test)]
mod tests {
    use crate::app_entry::Encoding;

    use super::*;

    #[test]
    fn returns_valid_app_entry() -> Result<(), ()> {
        let some_str = PathBuf::from("foo");
        let result = get_entry_from_file(some_str);
        assert!(result.is_ok());
        let app_entry = result.unwrap();
        assert_eq!(app_entry.app_type, AppType::Application);
        assert_eq!(app_entry.encoding, Encoding::UTF8);
        assert_eq!(app_entry.name, String::from("Firefox"));
        assert_eq!(
            app_entry.comment,
            Some(String::from(
                "A small descriptive blurb about this application."
            ))
        );
        assert_eq!(
            app_entry.icon,
            Some(String::from(
                "/Apps/firefox/browser/chrome/icons/default/default48.png"
            ))
        );
        assert_eq!(app_entry.exec, String::from("/Apps/firefox/firefox"));
        assert_eq!(app_entry.terminal, false);
        assert_eq!(app_entry.categories.len(), 3);
        Ok(())
    }
}
