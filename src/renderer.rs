use crate::domain::{Config, Post};
use anyhow::Result;
use std::path::Path;
use tera::{Context, Tera};

pub struct Renderer {
    tera: Tera,
}

impl Renderer {
    pub fn new<P: AsRef<Path>>(theme_dir: P) -> Result<Self> {
        let mut tera = Tera::new(theme_dir.as_ref().join("**/*.html").to_str().unwrap())?;
        tera.autoescape_on(vec![]); 
        Ok(Self { tera })
    }

    pub fn render_post(&self, post: &Post, config: &Config) -> Result<String> {
        let mut context = Context::new();
        context.insert("post", post);
        context.insert("config", config);
        Ok(self.tera.render("post.html", &context)?)
    }

    pub fn render_index(&self, posts: &[Post], config: &Config) -> Result<String> {
        let mut context = Context::new();
        context.insert("posts", posts);
        context.insert("config", config);
        Ok(self.tera.render("index.html", &context)?)
    }

    pub fn render_paginated_index(&self, paginator: &crate::pagination::Paginator<Post>, config: &Config) -> Result<String> {
        let mut context = Context::new();
        context.insert("paginator", paginator);
        context.insert("config", config);
        Ok(self.tera.render("index.html", &context)?)
    }

    pub fn render_taxonomy(&self, name: &str, posts: &[&Post], config: &Config) -> Result<String> {
        let mut context = Context::new();
        context.insert("name", name);
        context.insert("posts", posts);
        context.insert("config", config);
        Ok(self.tera.render("taxonomy.html", &context)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::PostMeta;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_render_paginated_index() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_dir = temp_dir.path().join("templates");
        fs::create_dir(&theme_dir).expect("Failed to create templates dir");

        let index_template = "Page {{ paginator.current_page }} of {{ paginator.total_pages }}: {% for post in paginator.items %}{{ post.meta.title }}{% endfor %}";
        fs::write(theme_dir.join("index.html"), index_template).expect("Failed to write template");

        let renderer = Renderer::new(&theme_dir).expect("Failed to create renderer");

        let paginator = crate::pagination::Paginator {
            current_page: 1,
            total_pages: 2,
            items: vec![Post {
                meta: PostMeta {
                    title: "P1".to_string(),
                    date: "2023".to_string(),
                    slug: "p1".to_string(),
                    tags: None,
                    categories: None,
                },
                content: "".to_string(),
            }],
        };

        let config = Config {
            title: "Blog".to_string(),
            base_url: "url".to_string(),
            description: None,
            posts_per_page: None,
        };

        let output = renderer.render_paginated_index(&paginator, &config).expect("Failed to render paginated index");
        assert!(output.contains("Page 1 of 2: P1"));
    }

    #[test]
    fn test_render_taxonomy() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_dir = temp_dir.path().join("templates");
        fs::create_dir(&theme_dir).expect("Failed to create templates dir");

        let taxonomy_template = "<h1>Tag: {{ name }}</h1><ul>{% for post in posts %}<li>{{ post.meta.title }}</li>{% endfor %}</ul>";
        fs::write(theme_dir.join("taxonomy.html"), taxonomy_template).expect("Failed to write template");

        let renderer = Renderer::new(&theme_dir).expect("Failed to create renderer");

        let post = Post {
            meta: PostMeta {
                title: "Hello World".to_string(),
                date: "2023-01-01".to_string(),
                slug: "hello-world".to_string(),
                tags: None,
                categories: None,
            },
            content: "content".to_string(),
        };

        let config = Config {
            title: "My Blog".to_string(),
            base_url: "https://example.com".to_string(),
            description: None,
            posts_per_page: None,
        };

        let output = renderer.render_taxonomy("rust", &[&post], &config).expect("Failed to render taxonomy");

        assert!(output.contains("<h1>Tag: rust</h1>"));
        assert!(output.contains("Hello World"));
    }

    #[test]
    fn test_render_post() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let theme_dir = temp_dir.path().join("templates");
        fs::create_dir(&theme_dir).expect("Failed to create templates dir");

        let post_template = r#"\
        <!DOCTYPE html>
        <html>
        <head><title>{{ post.meta.title }} - {{ config.title }}</title></head>
        <body>
            <h1>{{ post.meta.title }}</h1>
            <div class="content">{{ post.content }}</div>
        </body>
        </html>
        "#;
        fs::write(theme_dir.join("post.html"), post_template).expect("Failed to write template");

        let renderer = Renderer::new(&theme_dir).expect("Failed to create renderer");

        let post = Post {
            meta: PostMeta {
                title: "Hello World".to_string(),
                date: "2023-01-01".to_string(),
                slug: "hello-world".to_string(),
                tags: None,
                categories: None,
            },
            content: "<p>This is content</p>".to_string(),
        };

        let config = Config {
            title: "My Blog".to_string(),
            base_url: "https://example.com".to_string(),
            description: None,
            posts_per_page: None,
        };

        let output = renderer.render_post(&post, &config).expect("Failed to render post");

        assert!(output.contains("<title>Hello World - My Blog</title>"));
        assert!(output.contains("<h1>Hello World</h1>"));
        assert!(output.contains("<div class=\"content\"><p>This is content</p></div>"));
    }
}