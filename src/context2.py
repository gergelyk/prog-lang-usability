from contextlib import contextmanager
    
@contextmanager
def Connection(address):
    print(f"Connection open to {address}")
    try:
        yield 123
    finally:
        print("Connection closed")

with Connection("target") as conn:
    print("Doing something well")
    
with Connection("target") as conn:
    raise Exception("Doing something wrong")
