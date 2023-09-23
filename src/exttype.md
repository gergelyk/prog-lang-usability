# Extending Existing Types

**Objective**: Add methods to integer type.

### Python

Extending built-in class is not possible. Inharitance helps to workaround the problem.

```python
{{#include exttype.py}}
```

### Rust

Extending built-in types is possible, but how to do it for all integer types at once?

```rust
{{#include exttype.rs}}
```

### Crystal

Inheritance is available, but not easy in case of this example. Instead of that, method can be added to the existing type:

```crystal
{{#include exttype.cr}}
```
