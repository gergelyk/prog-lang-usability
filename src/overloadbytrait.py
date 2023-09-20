from abc import ABC, abstractmethod
from typing import Any

class Colorful(ABC):
    @abstractmethod
    def get_color(self):
        pass

class Apple(Colorful):
    def get_color(self):
        return "green"

class Sky(Colorful):
    def get_color(self):
        return "blue"

def get_color(x: Any):
    if isinstance(x, Colorful):
        return x.get_color()
    else:
        return "transparent"

# types known in advance
print(f"{get_color(Apple())=}")
print(f"{get_color(Sky())=}")
print(f"{get_color(123)=}")
print(f"{get_color('example')=}")

# types determined in runtime
items = [Apple(), Sky(), 123,  "example"]
for item in items:
    color = get_color(item)
    print(f"{item} is {color}")
