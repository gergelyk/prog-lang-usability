## Calling libc

**Objective**: Call getpid from libc. Print PID returned by the function.

### Python

```python
{{#include calllibc.py}}
```

Similar approach can be used for accessing other shared libraries.

### Rust

Rust compiler comes with pre-prepared bindings for libc. As for now, this is only available in unstable versions of the compiler. Compile as:

```sh
rustc --edition 2024 -Z unstable-options calllibc.rs
```

```rust
{{#include calllibc.rs}}
```

### Crystal

```crystal
{{#include calllibc.cr}}
```

Similar approach can be used for accessing other shared libraries.
