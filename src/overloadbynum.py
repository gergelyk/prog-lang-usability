from typing import Optional

def say_hello(firstname: str, surname: Optional[str] = None):
    if surname is None:
        print(f"Hi {firstname}!")
    else:
        print(f"Hello {firstname} {surname}!")

say_hello("John");
say_hello("Tom", "Brown");
