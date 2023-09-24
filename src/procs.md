# Multiprocessing

**Objectives**: Create a producer and consumer passing the data from one to another using FIFO.

### Python

```python
{{#include procs.py}}
```

### Rust

Standard library of Rust 1.73 doesn't seem to have good support for multiprocessing. With some effort this can be implemented using platform specific features. For instance fork & waitid syscalls in Unix.

### Crystal

Standard library of Crystal 1.8.2 doesn't seem to have good support for multiprocessing. With some effort this can be implemented using platform specific features. For instance fork & waitid syscalls in Unix.
