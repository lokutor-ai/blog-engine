use crate::config::load_config;
use crate::content::load_posts;
use crate::renderer::Renderer;
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn build_site<P: AsRef<Path>>(
    project_dir: P,
    output_dir: P,
) -> Result<()> {
    let project_dir = project_dir.as_ref();
    let output_dir = output_dir.as_ref();

    let config = load_config(project_dir.join("config.toml"))?;
    
    let theme_dir = project_dir.join("themes").join("default"); 
    let renderer = Renderer::new(theme_dir)?;

    let posts = load_posts(project_dir.join("content"))?;

    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;

    let index_html = renderer.render_index(&posts, &config)?;
    fs::write(output_dir.join("index.html"), index_html)?;

    let sitemap_xml = crate::seo::generate_sitemap(&posts, &config)?;
    fs::write(output_dir.join("sitemap.xml"), sitemap_xml)?;

    let rss_xml = crate::seo::generate_rss(&posts, &config)?;
    fs::write(output_dir.join("rss.xml"), rss_xml)?;

    let static_dir = project_dir.join("static");
    if static_dir.exists() {
        copy_recursive(&static_dir, output_dir)?;
    }

    for post in &posts {
        let post_html = renderer.render_post(post, &config)?;
        let post_slug = &post.meta.slug;
        let post_dir = output_dir.join("posts").join(post_slug);
        fs::create_dir_all(&post_dir)?;
        fs::write(post_dir.join("index.html"), post_html)?;
    }

    Ok(())
}

fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(src)?;
            let target_path = dst.join(relative_path);
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, target_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_site() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let project_dir = temp_dir.path().join("myblog");
        let output_dir = temp_dir.path().join("public");

        fs::create_dir_all(&project_dir.join("content/posts")).unwrap();
        fs::create_dir_all(&project_dir.join("themes/default")).unwrap();
        
        let config_content = r#"
            title = "Test Blog"
            base_url = "https://example.com"
            description = "A test blog"
        "#;
        fs::write(project_dir.join("config.toml"), config_content).unwrap();

        let post_content = r#"---
title: My Post
date: 2023-01-01
slug: my-post
---
# Hello
"#;
        fs::write(project_dir.join("content/posts/post.md"), post_content).unwrap();

        let index_template = "<h1>Index: {{ config.title }}</h1><ul>{% for post in posts %}<li>{{ post.meta.title }}</li>{% endfor %}</ul>";
        fs::write(project_dir.join("themes/default/index.html"), index_template).unwrap();

        let post_template = "<h1>{{ post.meta.title }}</h1><div>{{ post.content }}</div>";
        fs::write(project_dir.join("themes/default/post.html"), post_template).unwrap();

        fs::create_dir_all(project_dir.join("static/css")).unwrap();
        fs::write(project_dir.join("static/css/style.css"), "body { color: red; }").unwrap();

        build_site(&project_dir, &output_dir).expect("Failed to build site");

        assert!(output_dir.join("index.html").exists());
        assert!(output_dir.join("posts/my-post/index.html").exists()); 
        assert!(output_dir.join("css/style.css").exists());
        assert!(output_dir.join("sitemap.xml").exists());
        assert!(output_dir.join("rss.xml").exists());
        
        let css_content = fs::read_to_string(output_dir.join("css/style.css")).unwrap();
        assert_eq!(css_content, "body { color: red; }");
        
        let index_html = fs::read_to_string(output_dir.join("index.html")).unwrap();
        assert!(index_html.contains("Index: Test Blog"));
        assert!(index_html.contains("My Post"));

        let post_html = fs::read_to_string(output_dir.join("posts/my-post/index.html")).unwrap();
        assert!(post_html.contains("<h1>My Post</h1>"));
        assert!(post_html.contains("<h1>Hello</h1>"));
    }
}