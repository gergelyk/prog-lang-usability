# Extending Built-in Type

**Objective**: Add methods or attributes to a built-in type.

### Python

Extending built-in class is not possible. Inharitance helps to workaround the problem.

```python
{{#include extbuiltin.py}}
```

### Rust

Extending built-in types is possible, but how to do it for all integer types at once?

```rust
{{#include extbuiltin.rs}}
```

### Crystal

```crystal
{{#include extbuiltin.cr}}
```
