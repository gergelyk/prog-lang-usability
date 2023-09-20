#[derive(Default)]
struct SayHello {
    surname: Option<String>,
}

impl SayHello {
    fn default() -> Self {
        Self{
            ..Default::default()
        }
    }
    
    fn surname(mut self, val: &str) -> Self {
        self.surname = Some(val.to_string());
        self
    }
    
    fn call(&self, firstname: &str) {
        if let Some(surname) = &self.surname {
            println!("Hello {} {}!", firstname, surname);
        } else {
            println!("Hi {}!", firstname);                
        }
    }
}

fn main() {
    SayHello::default().call("John");
    SayHello::default().surname("Brown").call("Tom");
}
