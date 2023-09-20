use serde::{Serialize, Deserialize};

#[derive(Default)]
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    height: f64,
}

fn main() {
    let person_a = Person {name: "Greg".to_string(), height: 1.80};
    let json_text_a = serde_json::to_string(&person_a).unwrap();
    println!("{}", json_text_a);

    let json_text_b = r#"{"name":"Tom", "height": 1.85}"#;
    let person_b: Person = serde_json::from_str(&json_text_b).unwrap();
    println!("{:?}", person_b);
}
