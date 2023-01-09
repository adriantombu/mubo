use serde::Deserialize;
use std::fs;
use std::path::Path;

// TODO: remove unwrap
// TODO: readme
// TODO: add some logs
// TODO: add some tests
// TODO: add to homebrew
fn main() {
    let path = Path::new("config.toml");
    let data = fs::read_to_string(path).unwrap();
    let values = toml::from_str::<Links>(&data).unwrap();

    let build_path = Path::new("r");
    prepare_build_directory(build_path);

    values.links.iter().for_each(|link| {
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
        fs::create_dir_all(&link_path).unwrap();
        fs::write(format!("{}/index.html", link_path), html.as_bytes()).unwrap();
    })
}

fn prepare_build_directory(build_path: &Path) {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path).unwrap();
    }

    fs::create_dir_all(build_path).unwrap()
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
