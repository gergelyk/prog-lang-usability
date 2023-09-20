from typing import Literal

Something = Literal["on", "off"]

def foo(*, something: Something):
    if something == "on":
        print("Something is ON")
    else:
        print("Something is OFF")

foo(something="on")
foo(something="off")
