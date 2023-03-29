# Doc-generator

Here is the documentation of the language in markdown, and a program to
generate static HTML files from them.

## `src` folder

Rust code to generate HTML files from MD files.

The binary is called like:

```sh
generator --input /path/to/markdown/folder/ --output /path/to/static/folder
```

## `markdown` folder

Contains the Markdown. All files inside are expected to be UTF-8 encoded
markdown, and have the `.md` file extension.

## `static` folder

Contains CSS, JS, and HTML templates. Here the MD files are written to
after being converted.

## `dist` folder

Coming soon, this folder will contain all HTML, CSS & JS minified after
running a script.
