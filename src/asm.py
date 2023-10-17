from cffi import FFI
ffi = FFI()
ffi.set_source("_pidlib", r"""
int get_pid() {
    int pid;
    asm ("movl $20, %%eax\n"
        "int $0x80\n"
        "movl %%eax, %0\n" : "=r"(pid));
    return pid;
}
""")
ffi.cdef("int get_pid();")
ffi.compile()

from _pidlib.lib import get_pid

print(get_pid())
