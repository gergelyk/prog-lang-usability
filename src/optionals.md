# Arguments of Optional Type

**Objective**: Define a fucntion that takes argument `x` of `Optional[int]` type. Return `123` if `x` is `None`. Otherwise return `x`.

Caution: don't confuse optional arguments (that can be skipped at the function call) with arguments of `Optional` type (that cannot be skipped, but can take value of `None`).

### Python

```python
{{#include optionals.py}}
```

### Rust

```rust
{{#include optionals.rs}}
```

Alternatively, `Into` allows for a convenient definition where `Some`can be skipped at the function call:

```rust
{{#include optionals2.rs}}
```

### Crystal

```crystal
{{#include optionals.cr}}
```

Arguments of `Bool` type need special attention. Short syntax above can provide unintended results. Universal alternative:

```crystal
{{#include optionals2.cr}}
```
