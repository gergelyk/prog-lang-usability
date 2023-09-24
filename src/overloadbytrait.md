# Dispatching by Interface

**Objective**: Define a fucntion that:

- For arguments that implements `get_color` method, calls this method and prints the result.
- For remaining arguments prints `transparent`.

Test it where type of the argument is known in advance time and when it determined only in runtime.

### Python

Abstract class can be used to define interface:

```python
{{#include overloadbytrait.py}}
```

Alternatively, existence of the methods can be checked in runtime without defining interfaces:

```python
{{#include overloadbytrait2.py}}
```

Yet another way of implementing `get_color`:

```python
{{#include overloadbytrait3.py}}
```

### Rust

```rust
{{#include overloadbytrait.rs}}
```

### Crystal

Abstract struct can be used to define interface:

```crystal
{{#include overloadbytrait.cr}}
```

Alternatively, existence of the methods can be checked in runtime without defining interfaces:

```crystal
{{#include overloadbytrait2.cr}}
```
