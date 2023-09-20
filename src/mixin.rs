trait Named {
    fn name(&self) -> &str;
}

trait Greeting : Named {
    fn hello(&self) {
        println!("Hello, my name is {}. How can I help you?", self.name());
    }
}

trait Farewell {
    fn bye(&self) {
        println!("Thank you. Bye!");
    }
}

struct ChatBot {
    name : String
}

impl Greeting for ChatBot {}
impl Farewell for ChatBot {}
impl Named for ChatBot {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
  let bot = ChatBot{name: "R2D2".to_string()};
  bot.hello();
  bot.bye();
}
