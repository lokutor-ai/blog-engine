use crate::domain::{Config, Post};
use anyhow::Result;

pub fn generate_sitemap(posts: &[Post], config: &Config) -> Result<String> {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    xml.push_str(&format!("  <url><loc>{}/</loc></url>\n", config.base_url.trim_end_matches('/')));

    for post in posts {
        xml.push_str(&format!(
            "  <url><loc>{}/posts/{}/</loc></url>\n",
            config.base_url.trim_end_matches('/'),
            post.meta.slug
        ));
    }

    xml.push_str("</urlset>");
    Ok(xml)
}

pub fn generate_rss(posts: &[Post], config: &Config) -> Result<String> {
    let mut rss = String::from(r#"<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">
<channel>
"#);

    rss.push_str(&format!("  <title>{}</title>\n", config.title));
    rss.push_str(&format!("  <link>{}</link>\n", config.base_url.trim_end_matches('/')));
    rss.push_str(&format!("  <description>{}</description>\n", config.description.as_deref().unwrap_or("")));

    for post in posts {
        rss.push_str("  <item>\n");
        rss.push_str(&format!("    <title>{}</title>\n", post.meta.title));
        rss.push_str(&format!(
            "    <link>{}/posts/{}/</link>\n",
            config.base_url.trim_end_matches('/'),
            post.meta.slug
        ));
        rss.push_str(&format!("    <guid>{}/posts/{}/</guid>\n", config.base_url.trim_end_matches('/'), post.meta.slug));
        rss.push_str(&format!("    <pubDate>{}</pubDate>\n", post.meta.date));
        rss.push_str("  </item>\n");
    }

    rss.push_str("</channel>\n</rss>");
    Ok(rss)
}

use serde::Serialize;

#[derive(Serialize)]
struct SearchItem {
    title: String,
    slug: String,
    content: String,
}

pub fn generate_search_index(posts: &[Post]) -> Result<String> {
    let search_items: Vec<SearchItem> = posts
        .iter()
        .map(|p| SearchItem {
            title: p.meta.title.clone(),
            slug: p.meta.slug.clone(),
            content: p.content.clone(),
        })
        .collect();

    Ok(serde_json::to_string(&search_items)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::PostMeta;

    #[test]
    fn test_generate_sitemap() {
        let config = Config {
            title: "Test Blog".to_string(),
            base_url: "https://example.com".to_string(),
            description: None,
            posts_per_page: None,
        };

        let posts = vec![
            Post {
                meta: PostMeta {
                    title: "Post 1".to_string(),
                    date: "2023-01-01".to_string(),
                    slug: "post-1".to_string(),
                    tags: None,
                    categories: None,
                    draft: None,
                    image: None,
                },
                content: "content".to_string(),
            },
        ];

        let sitemap = generate_sitemap(&posts, &config).expect("Failed to generate sitemap");

        assert!(sitemap.contains("<loc>https://example.com/</loc>"));
        assert!(sitemap.contains("<loc>https://example.com/posts/post-1/</loc>"));
    }

    #[test]
    fn test_generate_rss() {
        let config = Config {
            title: "Test Blog".to_string(),
            base_url: "https://example.com".to_string(),
            description: Some("A test blog".to_string()),
            posts_per_page: None,
        };

        let posts = vec![
            Post {
                meta: PostMeta {
                    title: "Post 1".to_string(),
                    date: "2023-01-01".to_string(),
                    slug: "post-1".to_string(),
                    tags: None,
                    categories: None,
                    draft: None,
                    image: None,
                },
                content: "content".to_string(),
            },
        ];

        let rss = generate_rss(&posts, &config).expect("Failed to generate RSS");

        assert!(rss.contains("<title>Test Blog</title>"));
        assert!(rss.contains("<link>https://example.com</link>"));
                assert!(rss.contains("<title>Post 1</title>"));
                assert!(rss.contains("https://example.com/posts/post-1/"));
            }
        
            #[test]
            fn test_generate_search_index() {
                let posts = vec![
                    Post {
                        meta: PostMeta {
                            title: "Searchable Post".to_string(),
                            date: "2023-01-01".to_string(),
                            slug: "searchable-post".to_string(),
                            tags: Some(vec!["rust".to_string()]),
                            categories: None,
                            draft: None,
                            image: None,
                        },
                        content: "This is searchable content".to_string(),
                    },
                ];
        
                let index_json = generate_search_index(&posts).expect("Failed to generate search index");
                assert!(index_json.contains("Searchable Post"));
                assert!(index_json.contains("searchable-post"));
            }
        }
        