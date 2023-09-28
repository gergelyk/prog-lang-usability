# Inspection

**Objective**: Extend class by a method that will print names and types of the instance variables.

### Python

Based on type hints:

```python
{{#include inspect.py}}
```

Based the values of variables:

```python
{{#include inspect2.py}}
```

Note that there are more corner cases to be considered in Python. For instance how to differentiate instance variables and methods. Whether dunder variables should be considered or not, etc.  

### Rust

```rust
{{#include inspect.rs}}
```

### Crystal

```crystal
{{#include inspect.cr}}
```
