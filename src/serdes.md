# Serdes

**Objective**: Define a simple class and create an instance. Serialize the object into JSON. Finally deserialize JSON into new object.

### Python

```python
{{#include serdes.py}}
```
Notes regarding deserialization:
- Types are not checked. Use [dacite](https://pypi.org/project/dacite/) or [pydantic](https://pypi.org/project/pydantic/) if needed.
- Extra arguments raise an exception.
- Missing arguments raise an exception, unless default values are provided.

### Rust

Stdlib doesn't support serialization/deserialization to/from JSON. Crate [serde](https://docs.rs/serde/latest/serde/) has been used.


```rust
{{#include serdes_rs/src/main.rs}}
```

Notes regarding deserialization:
- Incorrect types result in error.
- Extra arguments are allowed and ignored.
- Missing arguments result in error.

### Crystal

```rust
{{#include serdes.cr}}
```

Notes regarding deserialization:
- Incorrect types raise an exception.
- Extra arguments are allowed and ignored.
- Missing arguments raise an exception, unless default values are provided.
