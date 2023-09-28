# Rust - thiserror

This chapter presents usage of [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) crate.

### Objectives

- Implement a chain of functions where one calls another.
- Return an error from the innermost function.
- Add context to the errors on each subsequent level.
- Handle the error on the top level by displaying all the causes.
- Consider `std::io::Error` as well as user-defined error as a root cause.

### Implementation

```rs
// config.rs
{{#include thiserror_demo/src/config.rs}}
```

```rs
// setup.rs
{{#include thiserror_demo/src/setup.rs}}
```

```rs
// launcher.rs
{{#include thiserror_demo/src/launcher.rs}}
```

```rs
// main.rs
{{#include thiserror_demo/src/main.rs}}
```

### Testing

```
$ cargo build
$ ./target/debug/thiserror_demo
Failed to setup application
  Because: Failed to configure
  Because: Failed to read config file
  Because: No such file or directory (os error 2)

$ touch myconfig
$ ./target/debug/thiserror_demo
Failed to setup application
  Because: Failed to configure
  Because: Failed to parse config file: Unknown key 'foo'
```

### Open Topics
- add `impl Debug for T` for each error that derive(DebugStack)
- add config file path to `FileReadError`

