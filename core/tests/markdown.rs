use yugi::markdown::markdown_to_html;

#[test]
fn test_markdown_to_html() {
    let markdown = "# Hello, World!";
    let expected_html = "<h1>Hello, World!</h1>\n";
    assert_eq!(markdown_to_html(markdown), expected_html);
}
