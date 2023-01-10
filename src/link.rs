use crate::templates::Template;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Links<'a> {
    pub links: Vec<Link<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Link<'a> {
    pub key: &'a str,
    pub url: &'a str,
}

impl Link<'_> {
    pub fn render(&self, tt: &Template, build_path: &Path) -> Result<()> {
        let mut context = HashMap::new();
        context.insert("url", self.url);

        let html = tt.render("link", &context)?;

        let link_path = format!("{}/{}", build_path.display(), self.key);
        fs::create_dir_all(&link_path)
            .with_context(|| format!("Failed to create the directory at path {}", link_path))?;

        fs::write(format!("{}/index.html", link_path), html.as_bytes())
            .with_context(|| format!("Failed to create the file at path {}/index.html", link_path))
    }
}
