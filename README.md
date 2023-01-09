# Mubo, a static url shortener generator

**Mubo** allows you to host your own static url shortener on GitHub Pages (or Netlify, Vercel, GitLab Pages, ...) by populating a `config.toml` file.

## Getting started

### Add a new link

To add a new link, append the following values to your config.toml file:

```toml
[[links]]
key = "otso"
url = "https://otso.fr"
```

The `key` represents the slug that will be used to redirect to the `url`.

### Build the static site

Run the `mubo` command within the root directory of your project where lies the `config.toml` file.

It will generate all the pages and subdirectories under the `r` directory.

### Use the url shortener

If we take the example above, you will need to go to the `YOUR_BASE_URL/r/otso` to be redirected to `https://otso.fr`

### Live example

To see a live example of this script, you can have a look at [my own url shortener](https://github.com/adriantombu/go.otso.fr).

### Did you know ?

> mubo means 'short' in Cebunao