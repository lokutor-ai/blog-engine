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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::PostMeta;
    use std::fs;
    use tempfile::TempDir;

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
        };

        let output = renderer.render_post(&post, &config).expect("Failed to render post");

        assert!(output.contains("<title>Hello World - My Blog</title>"));
        assert!(output.contains("<h1>Hello World</h1>"));
        assert!(output.contains("<div class=\"content\"><p>This is content</p></div>"));
    }
}