from typing import Optional

def foo(x: Optional[int]):
    #x = x or 123  # not valid, foo(0) would give 321
    x = 123 if x is None else x
    print(x)

foo(321)  # 321
foo(0)    # 0
foo(None) # 123
