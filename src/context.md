# Managed Context

**Objective**: Allocate resources and call given code in context of those resources. Finally clean-up resources despite of the fact if the code signalled an error or not.

Note: In the examples below `123` represents allocated resources.

### Python

```python
{{#include context1.py}}
```

Alternatively:

```python
{{#include context2.py}}
```

### Rust

```rust
{{#include context.rs}}
```

Note: doesn't meet objectives in case of panic.

### Crystal

```crystal
{{#include context.cr}}
```
