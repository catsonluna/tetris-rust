use platform_dirs::AppDirs;

use crate::engine::utils::crypto;
fn get_app_data_dir() -> String {
    let app_dirs = AppDirs::new(Some("com.catsonluna.tetris"), true).unwrap();
    app_dirs.data_dir.to_str().unwrap().to_string()
}

pub fn load(path: &str) -> String {
    let path = format!("{}/{}", get_app_data_dir(), path);
    if let Some(parent_dir) = std::path::Path::new(&path).parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
    }
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn load_and_decrypt(path: &str) -> String {
    let data = load(path);
    if data.is_empty() {
        return String::new();
    }
    let bytes = base64::decode(data).unwrap();
    let decrypted = crypto::lib::decrypt(&bytes);
    String::from_utf8(decrypted).unwrap()
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

pub fn save_encrypted(path: &str, data: &str) {
    let encrypted = crypto::lib::encrypt(data.as_bytes());
    let encoded = base64::encode(encrypted);
    save(path, &encoded);
}

pub fn delete(path: &str) {
    let path = format!("{}/{}", get_app_data_dir(), path);
    if std::path::Path::new(&path).exists() {
        std::fs::remove_file(path).unwrap();
    }
}