fn main() {

    let mut animals: Vec<Box<dyn CanMakeSound>> = Vec::new();
    animals.push( Box::new(Cat{}) );
    animals.push( Box::new(Cow{}) );

    for animal in animals {
    animal.make_sound();
    }

}
