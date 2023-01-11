mod index;
mod link;
mod templates;

use crate::index::Index;
use crate::link::Links;
use crate::templates::Template;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    prepare_build_directory("r")?;

    let data = fs::read_to_string("config.toml").context("Failed to read the config.toml file")?;
    let values = Links::new(&data)?;
    let template = Template::new()?;

    values
        .links
        .iter()
        .try_for_each(|link| -> Result<()> { link.render(&template, "r") })?;

    Index::render(&template, &values.links)
}

fn prepare_build_directory(build_path: &str) -> Result<()> {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path)
            .with_context(|| format!("Failed to delete the directory at path {}", build_path))?;
    }

    if Path::new("./index.html").exists() {
        fs::remove_file("./index.html").context("Failed to delete file at path ./index.html")?;
    }

    fs::create_dir_all(build_path)
        .with_context(|| format!("Failed to create the directory at path {}", build_path))
}
