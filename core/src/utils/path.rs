use std::{
    env,
    path::{Path, PathBuf},
};

pub fn str_to_absolute_path(str_path: &str) -> PathBuf {
    if !Path::new(str_path).is_absolute() {
        let current_dir = env::current_dir().unwrap();
        return current_dir.join(str_path);
    }
    PathBuf::from(str_path)
}
