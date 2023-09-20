from abc import ABC, abstractmethod
from typing import Any

class Colorful(ABC):
    @abstractmethod
    def get_color():
        pass

class Car(Colorful):

Colorful()
