# Dynamic Dispatch

**Objective**: Create container accepting items of different types. Categorize items in runtime. Dispatch methods dynamically.

### Python

```python
{{#include dyndisp.py}}
```

Alternatively, instead of using Union type, classes `Cat` and `Cow` can have common base `Animal`.

### Rust

```rust
{{#include dyndisp.rs}}
```

Alternatively:

```rust
{{#include dyndisp2.rs}}
```

### Crystal

```crystal
{{#include dyndisp.cr}}
```

Alternatively:

```crystal
{{#include dyndisp2.cr}}
```
