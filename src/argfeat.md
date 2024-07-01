# Features of the Arguments

**Objectives**:
- Implement function with default values of arguments.
- Specify arguments by name in the function call.

### Python

```python
{{#include argfeat.py}}
```

Additionally Python also supports:
- Variadic positional args.
- Variadic keyword args.
- Keyword-only args.


### Rust

Example shows how to implement keyword arguments, but positional arguments are not available at the same time.

```rust
{{#include argfeat.rs}}
```

Notes:
- `default()`, `call()` and each setter needs to be called (builder pattern).
- Builder can also implement variadic args (by appending values to a vector) and variadic keyword args (by appending name and values to a hashmap).
- Separate names can be used externally and internally (e.g. field in `Foo` can have a different name than corresponding setter).

### Crystal

```crystal
{{#include argfeat.cr}}
```

Additionally Crystal supports:
- Variadic positional args.
- Variadic keyword args.
- Keyword-only args.
- Internal/external arg names.
