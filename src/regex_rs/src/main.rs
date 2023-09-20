use regex::Regex;

fn main() {
    let text = "Hello John!";
    let re = Regex::new(r"(\w+)\!$").unwrap();
    
    if let Some(m) = re.captures(text) {
        println!("Name: {}", &m[1]);
    }
}
