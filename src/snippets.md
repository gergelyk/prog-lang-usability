# Snippets

Snippets presented in this book are available in the [repo](https://github.com/gergelyk/prog-lang-usability). Most of them are single files that can be directly run or compiled. The goal is to use standard libraries unless it is missing some essential functionality. In remaining cases we use third-party libraries and prioritize those that are the most popular.

## Python

Code is prepared mostly for CPython ~3.8. Most of the snippets contain mypy-style type hints. However, in some cases they are skipped for clarity.

File-based examples can be run as: `python3 example.py`

Project-based examples can be run as:

```
cd example_py
poetry install
poetry run python -m example
```

Which requires [poetry](https://python-poetry.org/) to be installed.

For checking type hints, [mypy](https://mypy.readthedocs.io/en/stable/index.html#) can be used.

## Rust

Code is prepared mostly for edition 2021, `rustc` ~1.73.

File-based examples can be compiled & run as: `rustc example.rs; ./example`

Project-based examples can be run as: `cd example_rs; cargo run`

For clarity, some of the examples employ unsafe constructs like `unwrap`. More sophisticated error handling may be desired in the final applications. Also some of the examples use `String` type despite of its impact on performance.

## Crystal

Code is prepared mostly for `crystal` ~1.8.

File-based examples can be compiled & run as: `crystal run example.cr`

Project-based examples can be run as: `cd example_cr; shards run`
