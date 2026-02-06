use crate::domain::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let content = fs::read_to_string(path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config() {
                        let toml_content = r#"
                            title = "My Test Blog"
                            base_url = "https://example.com"
                            description = "A test blog"
                            posts_per_page = 10
                            theme = "default"
                        "#;
                
                        let mut file = NamedTempFile::new().expect("Failed to create temp file");
                        write!(file, "{}", toml_content).expect("Failed to write to temp file");
                        
                        let config = load_config(file.path()).expect("Failed to load config");
                
                        assert_eq!(config.title, "My Test Blog");
                        assert_eq!(config.base_url, "https://example.com");
                        assert_eq!(config.description, Some("A test blog".to_string()));
                        assert_eq!(config.posts_per_page, Some(10));
                        assert_eq!(config.theme, Some("default".to_string()));
                    }
                }
                