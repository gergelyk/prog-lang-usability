# Asynchronous Execution

**Objectives**: Create vector of data items to be processed. Spawn concurent coroutines, one for each data item. Collect and present results. 

### Python

```python
{{#include async.py}}
```

### Rust

```rust
{{#include async_rs/src/main.rs}}
```

In context of concurrency in Rust it is hard not to mention [tokio](https://tokio.rs/). This crate provides hybrid solution utilizing threads for executing asynchronous functions. Simple example can be found [here](./tokio.md).

### Crystal

```crystal
{{#include async.cr}}
```
