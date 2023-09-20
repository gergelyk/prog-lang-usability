# System Command

**Objectives**: Call system command `ls`. If return code is other than `0`, signal error that includes message obtained from stderr. Otherwise read stdout, split lines and present as a list.

### Python

```python
{{#include cmd.py}}
```

Note: in practice, packages like `plumbum` can be a good alternative.

### Rust

```rust
{{#include cmd.rs}}
```

Note: `unwrap` has been used for simplicity. In practice this can be replaced by error propagation.

### Crystal

```crystal
{{#include cmd1.cr}}
```

Alternative variant below forwards stderr of the subprocess to stderr of the main process. Doesn't meet objectives but is still worth of mentioning.

```crystal
{{#include cmd2.cr}}
```
