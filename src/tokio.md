# Rust - tokio

This chapter presents simple usage of [`tokio`](https://tokio.rs/) crate.

**Objectives**: Create vector of 10 data items to be processed. Spawn concurent coroutines, one for each data item. Execute them in a set of 5 threads. Collect and present results. 

### Implementation

```rs
// main.rs
{{#include tokio_demo/src/main.rs}}
```
