# Multithreading

**Objectives**: Create queue of data to be processed and queue of processing results. Spawn T threads, where each thread is a worker that performs data processing.

### Python

CPython 3.8.10 has GIL (global interpreter lock) that prevents threads from running in parallel. However multiple threads can be spawned and run concurently.

```python
{{#include thre.py}}
```

In practice [`ThreadPoolExecutor`](https://docs.python.org/dev/library/concurrent.futures.html#threadpoolexecutor) can efficiently simplify this code. However, for the sake of this exercise we use `Thread` in order to explore more of this field.

### Rust

Rust supports threads that can be run simultaneously. Crate [flume](https://crates.io/crates/flume) has been used as a replacement of `std::sync::mpsc`. Flume provides multiple-producers-multiple-consumers channels. Additionally crate [rand](https://crates.io/crates/rand) provides random numbers for demonstrational purposes.

```rust
{{#include thre_rs/src/main.rs}}
```

In context of multithreading in Rust it is hard not to mention [tokio](https://tokio.rs/). This crate provides hybrid solution utilizing threads for executing asynchronous functions. Simple example can be found [here](./tokio.md).

### Crystal

Crystal 1.8.2 doesn't support multithreading. Entire application runs in a single thread, except for garbage collector which runs in a separate thread.

