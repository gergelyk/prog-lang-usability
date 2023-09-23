# Multiprocessing

**Objectives**: Create a producer and consumer passing the data from one to another using FIFO.

### Python

```python
{{#include procs.py}}
```

### Rust

Work in progress...

### Crystal

Crystal 1.8.2 doesn't support multiprocessing. With some effort this can be implemented using fork & waitid syscalls.
