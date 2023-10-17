# Making syscall

**Objective**: Invoke `getpid` syscall. Print returned PID.

### Python

There is no dedicated binding for `syscall` in standard library. However `syscall` from `libc` can be utilized:

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
