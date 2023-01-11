use crate::link::Link;
use crate::templates::Template;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

pub struct Index;

impl Index {
    pub fn render(tt: &Template, links: &[Link]) -> Result<()> {
        let mut context = HashMap::new();
        context.insert("links", links);

        let html = tt.render("index", &context)?;
        fs::write("./index.html", html.as_bytes()).context("Failed to write to index.html")?;
        fs::write("./r/index.html", "").context("Failed to write to ./r/index.html")
    }
}
