use std::{env, fs, path::Path};

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_new_command_generate_template() {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let project_dir_path = Path::new("my_project");
    let index_md_path = project_dir_path.join("content/index.md");
    let style_css_path = project_dir_path.join("static/css/style.css");
    let base_html_path = project_dir_path.join("templates/base.html");

    // Change current directory to temp directory because yugi create template in executing directory.
    env::set_current_dir(temp_dir_path).unwrap();

    let stdout_result = format!(
        "Project created at {}\n",
        project_dir_path.to_string_lossy()
    );

    let mut cmd = Command::cargo_bin("yugi").unwrap();
    cmd.arg("new")
        .arg("my_project")
        .assert()
        .stdout(stdout_result)
        .success();

    let index_md = fs::read_to_string(index_md_path).unwrap();
    let style_css = fs::read_to_string(style_css_path).unwrap();
    let base_html = fs::read_to_string(base_html_path).unwrap();

    assert_eq!(index_md, "# Welcome to Yugi\n");
    assert_eq!(style_css, "/* Add your styles here */");
    assert_eq!(base_html, "<html><body>{{ content }}</body></html>");
}

#[test]
fn test_new_command_without_project_name_should_return_error() {
    let mut cmd = Command::cargo_bin("yugi").unwrap();
    cmd.arg("new").assert().failure();
}
