#![feature(auto_traits)]
#![feature(negative_impls)]

auto trait Transparent {
}

trait Colorful {
    fn get_color(&self) -> &'static str;
}

struct Apple;

impl !Transparent for Apple {}
impl Colorful for Apple {
  fn get_color(&self) -> &'static str {
    "green"
  }
}
    
struct Sky;

impl !Transparent for Sky {}
impl Colorful for Sky {
  fn get_color(&self) -> &'static str {
    "blue"
  }
}

impl<T> Colorful for T
where
    T: Transparent
{
  fn get_color(&self) -> &'static str {
    "transparent"
  }
}

fn get_color(x: impl Colorful) -> &'static str {
    x.get_color()
}


fn main() {
    // types known in compile time
    dbg!( get_color(Apple{}) );
    dbg!( get_color(Sky{}) );
    dbg!( get_color(123) );
    dbg!( get_color("example") );

    // types determined in runtime
    let items: Vec<&dyn Colorful> = vec![&Apple{}, &Sky{}, &123, &"example"];
    for item in items {
        // Below we print only color of the item. Printing the items themself
        // needs `fmt` to be implemented and is out of the scope of this example.
        println!("item is {}", item.get_color() );
    }
}
