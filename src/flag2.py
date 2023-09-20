from enum import Enum

class Something(Enum):
    on = "ON"
    off = "OFF"
    
def foo(something: Something):
    if something == Something.on:
        print("Something is ON")
    else:
        print("Something is OFF")

foo(Something.on)
foo(Something.off)
