use std::path::Path;

pub async fn new_template(project_name: &str) {
    let project_path = Path::new(project_name);

    // Create project structure
    setup_content_dir(project_path).await;
    setup_static_dir(project_path).await;
    setup_templates_dir(project_path).await;

    println!("Project created at {}", project_path.to_string_lossy());
}

async fn setup_content_dir(project_path: &Path) {
    // Create content directory
    let content_dir_path = project_path.join("content");
    tokio::fs::create_dir_all(&content_dir_path).await.unwrap();

    // Create content content/index.md example file
    tokio::fs::write(&content_dir_path.join("index.md"), "# Welcome to Yugi\n")
        .await
        .unwrap()
}

async fn setup_static_dir(project_path: &Path) {
    // Create static and static/css directory
    let css_dir_path = project_path.join("static/css");
    tokio::fs::create_dir_all(&css_dir_path).await.unwrap();

    tokio::fs::write(
        &css_dir_path.join("style.css"),
        "/* Add your styles here */",
    )
    .await
    .unwrap();
}

async fn setup_templates_dir(project_path: &Path) {
    // Create static and static/css directory
    let templates_dir_path = project_path.join("templates");
    tokio::fs::create_dir_all(&templates_dir_path)
        .await
        .unwrap();

    tokio::fs::write(
        &templates_dir_path.join("base.html"),
        "<html><body>{{ content }}</body></html>",
    )
    .await
    .unwrap();
}
