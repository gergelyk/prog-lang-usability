# Usability of Programming Languages

This book can be viewed at: https://gergelyk.github.io/prog-lang-usability/

## Rendering

```sh
cargo install mdbook@0.4.32
mdbook serve -o
```

## Adding new language

- Generate custom [highlight.js](https://highlightjs.org/download)
- Copy `highlight.min.js` to `theme/highlight.js`
- Make sure to add corresponding section in `src/languages.md`, `src/snippets.md`

## Contributions

Pull requests are welcome.
