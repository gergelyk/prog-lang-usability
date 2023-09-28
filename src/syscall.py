# inspiredy by: https://stackoverflow.com/a/37032683

import ctypes
from enum import IntEnum

class Syscall(IntEnum):
    GETPID = 39

libc = ctypes.CDLL(None)
# or alternatively:
#libc = ctypes.cdll.LoadLibrary("libc.so.6") 

syscall = libc.syscall
pid = syscall(Syscall.GETPID)
print(pid)
