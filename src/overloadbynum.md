# Overloading Function Based on Number of Arguments

**Objective**: Define a fucntion that:

- Takes required argument, `firstname`
- Takes optional argument, `surname`
- Prints different messages depending on whether `surname` is provided or not.

### Python

```python
{{#include overloadbynum.py}}
```

### Rust

Here is where builder pattern comes in:

```rust
{{#include overloadbynum.rs}}
```

### Crystal

Variant 1: Presence of `surname` is checked in compile time.

```crystal
{{#include overloadbynum.cr}}
```

Variant 2: Presence of `surname` is checked in runtime.

```crystal
{{#include overloadbynum2.cr}}
```
