pub mod app_entry;
mod app_finder;
mod constants;
mod desktop_file_parser;

pub fn derp() {
    if let Ok(apps) = app_finder::list_shared_apps() {
        for app in apps {
            println!("App {:?}", app.name);
        }
    } else {
        println!("shit!");
    }
}
