from contextlib import contextmanager

def div(a: float, b: float) -> float:
  if b == 0.0:
      raise ZeroDivisionError("Division by zero is illegal")
  return a / b

def div_and_add(a: float, b: float) -> float:
  n = div(a, b)
  m = div(b, a)
  s = m + n
  if s >= 10:
      raise OverflowError("Sum is too large")
  return s

def show(a, b):
    try:
        res = div_and_add(a, b)
    except ZeroDivisionError as exc:
        print(f"Zero division error: {exc}")
    except OverflowError as exc:
        print(f"Overflow error: {exc}")
    except Exception as exc:
        print(f"Error: {exc}")
    else:
        print(f"Result: {res}")

show(2.0, 4.0)   # Result: 2.5
show(0.0, 4.0)   # Zero division error: Division by zero is illegal
show(2.0, 0.0)   # Zero division error: Division by zero is illegal
show(100.0, 1.0) # Overflow error: Sum is too large
