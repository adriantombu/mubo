use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

// TODO: readme
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

    values.links.iter().try_for_each(|link| -> Result<()> {
        let html = format!(
            r#"
        <!DOCTYPE html>
        <html>
            <head>
                <link rel="canonical" href="{}" />
                <meta http-equiv="content-type" content="text/html; charset=utf-8" />
                <meta http-equiv="refresh" content="0;url={}" />
            </head>
            <body>
                <h1>Redirecting...</h1>
                  <a href="{}">Click here if you are not redirected.</a>
                  <script>location.href="{}"</script>
            </body>
        </html>
        "#,
            link.url, link.url, link.url, link.url
        );

        let link_path = format!("{}/{}", build_path.display(), link.key);
        fs::create_dir_all(&link_path)
            .with_context(|| format!("Failed to create the directory at path {}", link_path))?;
        fs::write(format!("{}/index.html", link_path), html.as_bytes()).with_context(|| {
            format!("Failed to create the file at path {}/index.html", link_path)
        })?;

        Ok(())
    })?;

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

    fs::create_dir_all(build_path).with_context(|| {
        format!(
            "Failed to create the directory at path {}",
            build_path.display()
        )
    })
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub links: Vec<Link>,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub key: String,
    pub url: String,
}
