use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Access {
    Shared,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppType {
    Application,
    Link,
    Directory,
}

impl Into<String> for AppType {
    fn into(self) -> String {
        match self {
            Self::Directory => "Directory",
            Self::Application => "Application",
            Self::Link => "Link",
        }
        .to_string()
    }
}

impl From<String> for AppType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Directory" => Self::Directory,
            "Link" => Self::Link,
            "Application" => Self::Application,
            _ => panic!("Invalid app type!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Encoding {
    UTF8,
    Other(String),
}

/// [Desktop Entry] the Desktop Entry group header identifies the file as a desktop entry
/// Type(app_type) the type of the entry, valid values are Application, Link and Directory
/// Encoding the character encoding of the desktop file
/// Name the application name visible in menus or launchers
/// Comment a description of the application used in tooltips
/// Icon the icon shown for the application in menus or launchers
/// Exec the command that is used to start the application from a shell.
/// Terminal whether the application should be run in a terminal, valid values are true or false
/// Categories semi-colon (;) separated list of menu categories in which the entry should be shown
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEntry {
    pub app_type: AppType,
    pub encoding: Encoding,
    pub name: String,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub exec: String,
    pub terminal: bool,
    pub categories: Vec<String>,
}

impl AppEntry {
    pub fn new() -> Self {
        Self {
            app_type: AppType::Application,
            encoding: Encoding::UTF8,
            name: String::new(),
            comment: None,
            icon: None,
            exec: String::new(),
            terminal: false,
            categories: vec![],
        }
    }
}

/// Useful for parsing
pub enum EntryTypes {
    AppTypeEntry(AppType),
    EncodingEntry(Encoding),
    Name(String),
    Comment(String),
    Icon(String),
    Exec(String),
    Terminal(bool),
    Categories(Vec<String>),
    Ignore,
}
