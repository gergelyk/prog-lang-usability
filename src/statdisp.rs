use std::any::Any;

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
    let mut animals: Vec<Box<dyn Any>> = Vec::new();
    animals.push( Box::new(Cat{}) );
    animals.push( Box::new(Cow{}) );

    for animal in animals {
        if let Some(cat) = animal.downcast_ref::<Cat>() {
            cat.make_sound();
        } else if let Some(cow) = animal.downcast_ref::<Cow>() {
            cow.make_sound();
        }
    }
}
