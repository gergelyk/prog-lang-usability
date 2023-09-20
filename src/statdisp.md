# Static Dispatch

**Objectives**: Create container accepting items of any type. Categorize items in runtime. Dispatch methods statically.

### Python

Following code works well but I'm unable to give good reasons for doing this in practice. Perhaps performance of this code may be compared with dynamic dispatching. This needs more investigation though.

```python
{{#include statdisp.py}}
```

CPython interpreters older than 3.10 don't support `match`, thus alternative would be:

```python
{{#include statdisp2.py}}
```

### Rust

```rust
{{#include statdisp.rs}}
```

### Crystal

```rust
{{#include statdisp.cr}}
```
