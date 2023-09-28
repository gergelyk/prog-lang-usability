# Making syscall

**Objective**: Call getpid syscall. Print PID returned by the function.

### Python

There is not dedicated binding for `syscall` in standard library. However `syscall` from `libc` can be easily created:

```python
{{#include syscall.py}}
```

### Rust

Example below uses [syscalls](https://docs.rs/syscalls/latest/syscalls/) crate.

```rust
{{#include syscall_rs/src/main.rs}}
```

### Crystal

Dedicated module [Syscall](https://crystal-lang.org/api/master/Syscall.html) is available. Bindings to individual syscalls must be created manually.

```crystal
{{#include syscall.cr}}
```
