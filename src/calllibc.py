from ctypes import cdll
libc = cdll.LoadLibrary("libc.so.6") 
print(libc.getpid())
