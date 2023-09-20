from typing import Union, List

class Cat:
    def make_sound(self):
        print("miao")
        
class Cow:
    def make_sound(self):
        print("mooo")

Animal = Union[Cat, Cow]
animals: List[Animal] = []
animals.append( Cat() )
animals.append( Cow() )

for animal in animals:
    animal.make_sound()
    
