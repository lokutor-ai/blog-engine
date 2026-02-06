use crate::domain::Post;
use crate::parser::parse_markdown;
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn load_posts<P: AsRef<Path>>(dir_path: P) -> Result<Vec<Post>> {
    let entries: Vec<_> = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();

    entries
        .into_par_iter()
        .map(|entry| {
            let content = fs::read_to_string(entry.path())
                .with_context(|| format!("Failed to read file: {:?}", entry.path()))?;
            
            let post = parse_markdown(&content)
                .with_context(|| format!("Failed to parse file: {:?}", entry.path()))?;
            
            Ok(post)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_posts_from_dir() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let posts_dir = temp_dir.path().join("posts");
        fs::create_dir(&posts_dir).expect("Failed to create posts dir");

        let post1_content = r#"---
title: Post 1
date: 2023-01-01
slug: post-1
---
# Content 1
"#;
        let post2_content = r#"---
title: Post 2
date: 2023-01-02
slug: post-2
---
# Content 2
"#;
        
        fs::write(posts_dir.join("post1.md"), post1_content).expect("Failed to write post1");
        fs::write(posts_dir.join("post2.md"), post2_content).expect("Failed to write post2");
        fs::write(posts_dir.join("ignored.txt"), "ignored").expect("Failed to write text file");

        let posts = load_posts(posts_dir).expect("Failed to load posts");

        assert_eq!(posts.len(), 2);
        
        let titles: Vec<String> = posts.iter().map(|p| p.meta.title.clone()).collect();
        assert!(titles.contains(&"Post 1".to_string()));
        assert!(titles.contains(&"Post 2".to_string()));
    }
}