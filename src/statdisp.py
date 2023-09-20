from typing import Any

class Cat:
    def make_sound(self):
        print("miao")
        
class Cow:
    def make_sound(self):
        print("mooo")

animals: list[Any] = []
animals.append( Cat() )
animals.append( Cow() )

cat_make_sound = Cat.make_sound
cow_make_sound = Cow.make_sound

for animal in animals:    
    match animal:
        case Cat():
            cat_make_sound(animal)
        case Cow():
            cow_make_sound(animal)
