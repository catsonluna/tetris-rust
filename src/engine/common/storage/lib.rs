use platform_dirs::AppDirs;

use crate::engine::managers::game_statics::read_game_statics;

fn get_app_data_dir() -> String {
    let app_dirs = AppDirs::new(Some(&read_game_statics().url), true).unwrap();
    app_dirs.data_dir.to_str().unwrap().to_string()
}

pub fn load(path: &str) -> String {
    let path = format!("{}/{}", get_app_data_dir(), path);
    println!("{}", path);
    if let Some(parent_dir) = std::path::Path::new(&path).parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
    }
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn save(path: &str, data: &str) {
    let path = format!("{}/{}", get_app_data_dir(), path);
    if let Some(parent_dir) = std::path::Path::new(&path).parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
    }
    std::fs::write(path, data).unwrap();
}

pub fn delete(path: &str) {
    let path = format!("{}/{}", get_app_data_dir(), path);
    if std::path::Path::new(&path).exists() {
        std::fs::remove_file(path).unwrap();
    }
}
