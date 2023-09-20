from typing import Any

class Apple:
    def get_color(self):
        return "green"

class Sky:
    def get_color(self):
        return "blue"


def get_color(x: Any):
    if hasattr(x, 'get_color'):
        return x.get_color()
    return 'transparent'
