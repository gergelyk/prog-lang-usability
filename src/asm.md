# Embedding Assembly

**Objective**: Use assembly code below to invoke `getpid` syscall. Obtain PID from `%eax` and print it to stdout.

```x86asm
movl $20, %eax
int $0x80
```

Note that this should work under most unix platforms running on x86. Alternative solution that works on x86-64 would be:

```x86asm
movl $39, %eax
syscall
```

### Python

Embedding assembly makes little sense in an interpreted language like Python where typical intention is to be platform-agnostic. Hovewer, for the sake of example we can use [CFFI](https://cffi.readthedocs.io/en/stable/) module that will compile C code on the fly. C code in turn will embed our assembly:

```python
{{#include asm.py}}
```

### Rust

AT&T syntax applies.

```rust
{{#include asm.rs}}
```

### Crystal

Intel syntax applies.

```crystal
{{#include asm.cr}}
```
