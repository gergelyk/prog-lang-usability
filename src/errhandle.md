# Handling Errors

**Objectives**: Write an application where errors can be signalled in multiple places.
Write common error handler that prints error message if error occures.
Otherwise display execution results. Error handler should be able to distinguish errors.

### Python

```python
{{#include errhandle.py}}
```

### Rust

```rust
{{#include errhandle1.rs}}
```

Note: in the example above, in the error handler there is no way to distinguish error types (other way than looking at the error message).

Alternatively:

```rust
{{#include errhandle2.rs}}
```

Note that there are crates in the wild which provide more sophisticated way of handling errors. See [thiserror](./thiserror.md) and [anyhow](./anyhow.md).

### Crystal

```crystal
{{#include errhandle.cr}}
```
