use std::path::{self, Path};
use std::{fs, io};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub async fn create_dir_if_not_exists(dir: impl AsRef<Path>) {
    if !fs::exists(&dir).unwrap() {
        tokio::fs::create_dir(&dir).await.unwrap();
    }
}
