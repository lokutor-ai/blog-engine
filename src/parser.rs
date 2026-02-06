use crate::domain::{Post, PostMeta};
use anyhow::{Context, Result};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};

pub fn parse_markdown(content: &str) -> Result<Post> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(content);

    let meta: PostMeta = result
        .data
        .ok_or_else(|| anyhow::anyhow!("No frontmatter found"))?
        .deserialize()
        .context("Failed to deserialize frontmatter")?;

    let mut html_output = String::new();
    let options = Options::all();
    let parser = Parser::new_ext(&result.content, options);
    html::push_html(&mut html_output, parser);

    Ok(Post {
        meta,
        content: html_output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_with_frontmatter() {
        let raw_content = r#"---
title: Hello World
date: 2023-10-27
slug: hello-world
---
# Welcome

This is a test post.
"#;

        let expected_meta = PostMeta {
            title: "Hello World".to_string(),
            date: "2023-10-27".to_string(),
            slug: "hello-world".to_string(),
            tags: None,
            categories: None,
            draft: None,
            image: None,
        };

        let result = parse_markdown(raw_content).expect("Failed to parse markdown");

        assert_eq!(result.meta, expected_meta);
        assert!(result.content.contains("<h1>Welcome</h1>"));
        assert!(result.content.contains("<p>This is a test post.</p>"));
    }
}