use std::path::Path;

use markdown::markdown_to_html;

pub mod markdown;
pub mod utils;

pub struct Page {
    pub name: String,
    pub html_content: String,
}

impl Page {
    pub fn new(name: String, html_content: String) -> Page {
        Page { name, html_content }
    }

    pub async fn from_markdown_file(file_path: &Path) -> Page {
        let md = tokio::fs::read_to_string(file_path).await.unwrap();
        let html_content = markdown_to_html(&md);
        let name = file_path.file_stem().unwrap().to_str().unwrap().to_string();
        Page::new(name, html_content)
    }
}
