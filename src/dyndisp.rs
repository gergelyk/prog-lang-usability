trait CanMakeSound {
    fn make_sound(&self);
}

struct Cat {
}

impl CanMakeSound for Cat {
    fn make_sound(&self) {
        println!("miao")
    }
}
    
struct Cow {
}

impl CanMakeSound for Cow {
    fn make_sound(&self) {
        println!("mooo")
    }
}

fn main() {

    let mut animals: Vec<&dyn CanMakeSound> = Vec::new();
    animals.push( &Cat{} );
    animals.push( &Cow{} );

    for animal in animals {
    animal.make_sound();
    }
 
}
