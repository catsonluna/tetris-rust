use platform_dirs::AppDirs;

fn get_app_data_dir() -> String {
    let app_dirs = AppDirs::new(Some("com.catsonluna.tetris"), true).unwrap();
    app_dirs.data_dir.to_str().unwrap().to_string()
}