def foo(x: int, y: int) -> int:
    return x + y

class C:
    def __init__(self, x: int):
        self.x = x
        
    def get_x(self) -> int:
        return self.x

foo(2, 3)

c = C(123)
print(c.get_x())
