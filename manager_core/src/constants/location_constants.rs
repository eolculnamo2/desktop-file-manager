use std::path::PathBuf;

// All users on system share apps here
const SHARED_APPS: &'static str = "/usr/share/applications";
pub const SHARED_TYPE: &'static str = "shared";

// Only current user has apps listed here
const USER_APPS: &'static str = "~/.local/share/applications";
pub const USER_TYPE: &'static str = "local";

pub fn get_user_app() -> PathBuf {
    expanduser::expanduser(USER_APPS).unwrap()
}

// pub fn get_route_from_user_path(appended_path: &str) -> PathBuf {
//     let app_dir = get_user_app();
//     app_dir.join(appended_path)
// }

// this is just here for consistency since user_app is computed on the fly
pub fn get_shared_app() -> &'static str {
    SHARED_APPS
}
