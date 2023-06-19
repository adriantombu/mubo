use crate::templates::Template;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Links {
    pub links: Vec<Link>,
}

impl Links {
    pub fn new(data: &str) -> Result<Links> {
        toml::from_str::<Links>(data).context("Failed to deserialize the config.toml file")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub key: String,
    pub url: String,
}

impl Link {
    pub fn render(&self, tt: &Template, build_path: &str) -> Result<()> {
        let mut context = HashMap::new();
        context.insert("url", self.url.as_str());

        let html = tt.render("link", &context)?;

        let link_path = format!("{}/{}", build_path, self.key);
        fs::create_dir_all(&link_path)
            .with_context(|| format!("Failed to create the directory at path {}", link_path))?;

        fs::write(format!("{}/index.html", link_path), html.as_bytes())
            .with_context(|| format!("Failed to create the file at path {}/index.html", link_path))
    }
}
