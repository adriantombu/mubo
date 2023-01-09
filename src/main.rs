mod templates;

use crate::templates::{LINK_INDEX, LINK_TEMPLATE};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tinytemplate::TinyTemplate;

// TODO: add some logs
// TODO: add some tests
// TODO: clean this shit
// TODO: add to homebrew
fn main() -> Result<()> {
    let path = Path::new("config.toml");
    let data = fs::read_to_string(path).context("Failed to read the config.toml file")?;
    let values =
        toml::from_str::<Links>(&data).context("Failed to deserialize the config.toml file")?;

    let build_path = Path::new("r");
    prepare_build_directory(build_path)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("link", LINK_TEMPLATE)
        .context("Failed to load the template LINK_TEMPLATE")?;
    tt.add_template("index", LINK_INDEX)
        .context("Failed to load the template LINK_INDEX")?;

    values.links.iter().try_for_each(|link| -> Result<()> {
        let context = LinkContext {
            url: link.url.clone(),
        };
        let html = tt.render("link", &context)?;

        let link_path = format!("{}/{}", build_path.display(), link.key);
        fs::create_dir_all(&link_path)
            .with_context(|| format!("Failed to create the directory at path {}", link_path))?;
        fs::write(format!("{}/index.html", link_path), html.as_bytes()).with_context(|| {
            format!("Failed to create the file at path {}/index.html", link_path)
        })?;

        Ok(())
    })?;

    let html = tt.render("index", &values)?;
    fs::write("./index.html", html.as_bytes()).context("Failed to write to index.html")?;

    Ok(())
}

fn prepare_build_directory(build_path: &Path) -> Result<()> {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path).with_context(|| {
            format!(
                "Failed to delete the directory at path {}",
                build_path.display()
            )
        })?;
    }

    if Path::new("./index.html").exists() {
        fs::remove_file("./index.html").context("Failed to delete file at path ./index.html")?;
    }

    fs::create_dir_all(build_path).with_context(|| {
        format!(
            "Failed to create the directory at path {}",
            build_path.display()
        )
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub key: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct LinkContext {
    pub url: String,
}
