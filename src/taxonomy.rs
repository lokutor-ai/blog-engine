use crate::domain::Post;
use std::collections::HashMap;

pub fn group_by_tag(posts: &[Post]) -> HashMap<String, Vec<&Post>> {
    let mut map = HashMap::new();
    for post in posts {
        if let Some(tags) = &post.meta.tags {
            for tag in tags {
                map.entry(tag.clone()).or_insert_with(Vec::new).push(post);
            }
        }
    }
    map
}

pub fn group_by_category(posts: &[Post]) -> HashMap<String, Vec<&Post>> {
    let mut map = HashMap::new();
    for post in posts {
        if let Some(categories) = &post.meta.categories {
            for category in categories {
                map.entry(category.clone()).or_insert_with(Vec::new).push(post);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::PostMeta;

    #[test]
    fn test_group_by_tag() {
        let posts = vec![
            Post {
                meta: PostMeta {
                    title: "P1".to_string(),
                    date: "2023".to_string(),
                    slug: "p1".to_string(),
                    tags: Some(vec!["rust".to_string(), "web".to_string()]),
                    categories: None,
                },
                content: "".to_string(),
            },
            Post {
                meta: PostMeta {
                    title: "P2".to_string(),
                    date: "2023".to_string(),
                    slug: "p2".to_string(),
                    tags: Some(vec!["rust".to_string()]),
                    categories: None,
                },
                content: "".to_string(),
            },
        ];

        let grouped = group_by_tag(&posts);
        assert_eq!(grouped.get("rust").unwrap().len(), 2);
        assert_eq!(grouped.get("web").unwrap().len(), 1);
    }
}