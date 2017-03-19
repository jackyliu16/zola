extern crate gutenberg;
extern crate tempdir;
extern crate glob;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

// use glob::glob;
use tempdir::TempDir;
use gutenberg::{Site};


#[test]
fn test_can_parse_site() {
    let mut path = env::current_dir().unwrap().to_path_buf();
    path.push("test_site");
    let site = Site::new(&path).unwrap();

    // Correct number of pages (sections are pages too)
    assert_eq!(site.pages.len(), 10);
    let posts_path = path.join("content").join("posts");

    // Make sure we remove all the pwd + content from the sections
    let basic = &site.pages[&posts_path.join("simple.md")];
    assert_eq!(basic.components, vec!["posts".to_string()]);

    // Make sure the page with a url doesn't have any sections
    let url_post = &site.pages[&posts_path.join("fixed-url.md")];
    assert!(url_post.components.is_empty());

    // Make sure the article in a folder with only asset doesn't get counted as a section
    let asset_folder_post = &site.pages[&posts_path.join("with-assets").join("index.md")];
    assert_eq!(asset_folder_post.components, vec!["posts".to_string()]);

    // That we have the right number of sections
    assert_eq!(site.sections.len(), 4);

    // And that the sections are correct
    let posts_section = &site.sections[&posts_path];
    assert_eq!(posts_section.subsections.len(), 1);
    assert_eq!(posts_section.pages.len(), 5);

    let tutorials_section = &site.sections[&posts_path.join("tutorials")];
    assert_eq!(tutorials_section.subsections.len(), 2);
    assert_eq!(tutorials_section.pages.len(), 0);

    let devops_section = &site.sections[&posts_path.join("tutorials").join("devops")];
    assert_eq!(devops_section.subsections.len(), 0);
    assert_eq!(devops_section.pages.len(), 2);

    let prog_section = &site.sections[&posts_path.join("tutorials").join("programming")];
    assert_eq!(prog_section.subsections.len(), 0);
    assert_eq!(prog_section.pages.len(), 2);
}

// 2 helper macros to make all the build testing more bearable
macro_rules! file_exists {
    ($root: expr, $path: expr) => {
        {
            let mut path = $root.clone();
            for component in $path.split("/") {
                path = path.join(component);
            }
            Path::new(&path).exists()
        }
    }
}

macro_rules! file_contains {
    ($root: expr, $path: expr, $text: expr) => {
        {
            let mut path = $root.clone();
            for component in $path.split("/") {
                path = path.join(component);
            }
            let mut file = File::open(&path).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            s.contains($text)
        }
    }
}

#[test]
fn test_can_build_site_without_live_reload() {
    let mut path = env::current_dir().unwrap().to_path_buf();
    path.push("test_site");
    let mut site = Site::new(&path).unwrap();
    let tmp_dir = TempDir::new("example").expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    site.build().unwrap();

    assert!(Path::new(&public).exists());

    assert!(file_exists!(public, "index.html"));
    assert!(file_exists!(public, "sitemap.xml"));
    assert!(file_exists!(public, "a-fixed-url/index.html"));

    assert!(file_exists!(public, "posts/python/index.html"));
    assert!(file_exists!(public, "posts/tutorials/devops/nix/index.html"));
    assert!(file_exists!(public, "posts/with-assets/index.html"));

    // Sections
    assert!(file_exists!(public, "posts/index.html"));
    assert!(file_exists!(public, "posts/tutorials/index.html"));
    assert!(file_exists!(public, "posts/tutorials/devops/index.html"));
    assert!(file_exists!(public, "posts/tutorials/programming/index.html"));
    // TODO: add assertion for syntax highlighting

    // No tags or categories
    assert_eq!(file_exists!(public, "categories/index.html"), false);
    assert_eq!(file_exists!(public, "tags/index.html"), false);

    // no live reload code
    assert_eq!(file_contains!(public, "index.html", "/livereload.js?port=1112&mindelay=10"), false);

    // Both pages and sections are in the sitemap
    assert!(file_contains!(public, "sitemap.xml", "<loc>https://replace-this-with-your-url.com/posts/simple</loc>"));
    assert!(file_contains!(public, "sitemap.xml", "<loc>https://replace-this-with-your-url.com/posts</loc>"));
}

#[test]
fn test_can_build_site_with_live_reload() {
    let mut path = env::current_dir().unwrap().to_path_buf();
    path.push("test_site");
    let mut site = Site::new(&path).unwrap();
    let tmp_dir = TempDir::new("example").expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    site.enable_live_reload();
    site.build().unwrap();

    assert!(Path::new(&public).exists());

    assert!(file_exists!(public, "index.html"));
    assert!(file_exists!(public, "sitemap.xml"));
    assert!(file_exists!(public, "a-fixed-url/index.html"));

    assert!(file_exists!(public, "posts/python/index.html"));
    assert!(file_exists!(public, "posts/tutorials/devops/nix/index.html"));
    assert!(file_exists!(public, "posts/with-assets/index.html"));

    // Sections
    assert!(file_exists!(public, "posts/index.html"));
    assert!(file_exists!(public, "posts/tutorials/index.html"));
    assert!(file_exists!(public, "posts/tutorials/devops/index.html"));
    assert!(file_exists!(public, "posts/tutorials/programming/index.html"));
    // TODO: add assertion for syntax highlighting

    // No tags or categories
    assert_eq!(file_exists!(public, "categories/index.html"), false);
    assert_eq!(file_exists!(public, "tags/index.html"), false);

    // no live reload code
    assert!(file_contains!(public, "index.html", "/livereload.js?port=1112&mindelay=10"));
}

#[test]
fn test_can_build_site_with_categories() {
    let mut path = env::current_dir().unwrap().to_path_buf();
    path.push("test_site");
    let mut site = Site::new(&path).unwrap();
    let tmp_dir = TempDir::new("example").expect("create temp dir");
    site.set_output_path(&tmp_dir);
    site.build().unwrap();
}

#[test]
fn test_can_build_site_with_tags() {
    let mut path = env::current_dir().unwrap().to_path_buf();
    path.push("test_site");
    let mut site = Site::new(&path).unwrap();
    let tmp_dir = TempDir::new("example").expect("create temp dir");
    site.set_output_path(&tmp_dir);
    site.build().unwrap();
}
