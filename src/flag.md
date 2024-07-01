# Boolean Arguments

**Objective**: Show how boolean arguments can be used to enable/disable features of a function. Focus on readability.

Note: Some of the concepts below can be extended to arguments that take more than two arbitrary values.

### Python

**Variant 1:**

```python
{{#include flag.py}}
```

Keyword-only arguments are used to enforce readability. Suffix like `_is_on` may feel verbose but provides clarity.

**Variant 2:**

```python
{{#include flag2.py}}
```

`Something` enum must be imported together with `foo` if they happen to be defined externally.

**Variant 3:**

```python
{{#include flag3.py}}
```

No need of importing `Something`. Keyword-only arguments are used to enforce readability.

### Rust

Using bool variable may not be a good practive. There are keyword arguments, so in invocation it wouldn't be visible what the flag is doing.

```rust
{{#include flag.rs}}
```

`Something` enum must be imported together with `foo` if they happen to be defined externally.

### Crystal

**Variant 1:**

```crystal
{{#include flag.cr}}
```

Keyword-only arguments are used to enforce readability. Suffix like `_is_on` may feel verbose but provides clarity.

**Variant 2:**

```crystal
{{#include flag2.cr}}
```

Keyword-only arguments are used to enforce readability. Suffix `_is_on` not needed and no need of importing `Something`.

Enum can be reused for other arguments.

**Variant 3:**

No need of importing `Something`. Readability is enforced despite of the fact that function is called with positional arguments.

```crystal
{{#include flag3.cr}}
```

Implementation of the function and of the enum looks too verbose.

Also note that enums in crystal can be mapped only to integers (no booleans).
