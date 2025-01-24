use std::{fs, path::Path};

use yugi::Page;

pub async fn build_project(project_dir: &Path, output_dir: &Path) {
    yugi::utils::fs::create_dir_if_not_exists(output_dir).await;
    let content_dir_path = project_dir.join("content");
    let entries = fs::read_dir(content_dir_path).unwrap();
    let mut content_files = vec![];
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            content_files.push(path);
        }
    }

    let mut pages: Vec<Page> = vec![];
    //todo: Do Parallel
    for c in content_files {
        let page = Page::from_markdown_file(c.as_path()).await;
        pages.push(page);
    }

    println!("project_dir Dir {:?}", project_dir);
    let template_path = project_dir.join("templates/base.html");
    let template = tokio::fs::read_to_string(template_path).await.unwrap();
    for mut page in pages {
        page.html_content = template.replace("{{ content }}", &page.html_content);
        println!("output_dir  {:?}", output_dir);
        let page_path = &output_dir.join(&page.name).with_extension("html");

        println!("page_path  {:?}", page_path);
        println!("name  {:?}", &page.name);

        tokio::fs::write(page_path, &page.html_content)
            .await
            .unwrap();
    }

    copy_static_files_to_output(project_dir, output_dir).await;
}

async fn copy_static_files_to_output(project_dir: &Path, output_dir: &Path) {
    let source_static_dir_path = &project_dir.join("static");
    let target_static_dir_path = &output_dir.join("static");

    yugi::utils::fs::create_dir_if_not_exists(target_static_dir_path).await;

    println!("source_static_dir_path {:?}", source_static_dir_path);
    println!("target_static_dir_path {:?}", target_static_dir_path);

    yugi::utils::fs::copy_dir_all(source_static_dir_path, target_static_dir_path).unwrap();
}
