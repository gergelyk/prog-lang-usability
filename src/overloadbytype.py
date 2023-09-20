from typing import Union
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

def flip(a: Union[bool, int, Point]):
    
    if isinstance(a, bool):
        return not a
    
    if isinstance(a, int):
        return -a
    
    if isinstance(a, Point):
        return Point(a.y, a.x)

    print(f"Warning: Type {a.__class__.__name__} not flippable");
    return a

# types known in advance
print(f"{flip(False)=}")
print(f"{flip(3)=}")
print(f"{flip(Point(5, 7))=}")
print(f"{flip(33.3)=}")

# types determined in runtime
items = [False, 3,  Point(5, 7), 33.3]
for item in items:
    flipped = flip(item)
    print(f"{item} flipped is {flipped}")
