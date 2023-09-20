## Mixins

**Objective**: Create methods that can be injected into classes of our choice. Methods should have access to the fields of the instances. Injection should not involve inheritance.

### Python

Python supports multiple-inheritance. It is natural to use it for this use case:

```python
{{#include mixin.py}}
```

Using inheritance hovewer violates definition of a mixin. If this is important for the design, we can copy methods without affecting MRO:

```python
{{#include mixin2.py}}
```

More sophisticated packages like [mixin](https://pypi.org/project/mixin/) can be used.

### Rust

Rust doesn't support inheritance. Traits are natural way of fulfilling objectives.

Note that associated functions can't directly access fields of the structure. For this reason we define helper trait `Named`.

```rust
{{#include mixin.rs}}
```

### Crystal

Crystal supports single-inheritance. Mixins can be implemented using modules. Methods in the module can directly access fields of the structure. This is checked at compile time.

```crystal
{{#include mixin.cr}}
```
