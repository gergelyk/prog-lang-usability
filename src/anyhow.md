# Rust - anyhow

This chapter presents usage of [`anyhow`](https://github.com/dtolnay/anyhow) crate.

### Objectives

- Implement a chain of functions where one calls another.
- Return an error from the innermost function.
- Add context to the errors on each subsequent level.
- Handle the error on the top level by displaying all the causes.
- Consider `std::io::Error` as well as user-defined error as a root cause.

### Implementation

```rs
// config.rs
{{#include anyhow_demo/src/config.rs}}
```

```rs
// setup.rs
{{#include anyhow_demo/src/setup.rs}}
```

```rs
// launcher.rs
{{#include anyhow_demo/src/launcher.rs}}
```

```rs
// main.rs
{{#include anyhow_demo/src/main.rs}}
```

### Testing

```
$ cargo build
$ ./target/debug/anyhow_demo
Error: Failed to setup application

Caused by:
    0: Failed to configure
    1: Failed to read config file: myconfig
    2: No such file or directory (os error 2)

$ touch myconfig
$ ./target/debug/anyhow_demo
Error: Failed to setup application

Caused by:
    0: Failed to configure
    1: Unknown key 'foo'
```
