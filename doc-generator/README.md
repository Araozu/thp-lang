# Doc-generator

Here is the documentation of the language in markdown, and a program to
generate static HTML files from them.

## `src` folder

Rust code to generate HTML files from MD files.

The binary is called like:

```sh
generator --input /path/to/markdown/folder/ --output /path/to/static/folder/
```

## `markdown` folder

Contains the Markdown. All files inside are expected to be UTF-8 encoded
markdown, and have the `.md` file extension.

### Indexing

`doc-generator` will not search for `.md` files. Instead, it will search for
`.toml` files, which index all the markdown files. This is used to generate
the file hierarchy and previous/next links in the documentation.

This file must be named `index.toml`, must be the only TOML file in its folder,
and must follow the following schema:

```toml
# Tipically index
entry-point = "file-without-extension"

[folder-1]
section-name = "Display name for the folder"
# Markdown files, without the .md extension
children = [
    "file1",
    "file2",
]
```

### Markdown extensions

The markdown follows the CommonMark specification, but certain code blocks
contain custom behaviour:

#### `meta`

A code block with language `meta` contains text in TOML format that indicates
metadata for the current page.

````toml
```meta
title: "Title of the page"
description: "Description of the page"
```
````

- title: Used to create the title of the page with the format `{title} - Misti`
- description: The description of the page, placed in a `<meta>` element in the `<head>`



## `static` folder

Contains CSS, JS, and HTML templates. Here the MD files are written to
after being converted.

There must be a `template.html` file inside this folder. This file will be used to generate the HTML from MD files.

Inside `template.html` there must be a string `{{markdown}}`:

```html
<!-- Some html -->
    {{markdown}}
<!-- More html -->
```

This string, `{{markdown}}`, will be replaced with the HTML generated
from Markdown

## `dist` folder

Coming soon, this folder will contain all HTML, CSS & JS minified after
running a script.
