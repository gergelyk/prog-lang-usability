from typing import Optional

def div(a: float, b: float) -> Optional[float]:
    if b == 0.0: return None
    return a / b

def div_and_add(a: float, b: float) -> Optional[float]:
    n = div(a, b)
    if n is None: return None

    m = div(b, a)
    if m is None: return None

    s = m + n
    if s >= 10: return None

    return s

print(div_and_add(2.0, 4.0))   # 2.5
print(div_and_add(0.0, 4.0))   # None
print(div_and_add(2.0, 0.0))   # None
print(div_and_add(100.0, 1.0)) # None
