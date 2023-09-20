#[derive(Debug)]
enum DivError {
  DivisionByZero(String),
}

#[derive(Debug)]
enum AddError {
  Overflow(String),
}

type DivResult = Result<f32, Box<dyn std::any::Any>>;

fn div(a: f32, b: f32) -> DivResult {
  if b == 0.0 {
    return Err(Box::new(DivError::DivisionByZero(
      "Division by zero is illegal".to_string())));
  }
  Ok(a / b)
}

fn div_and_add(a: f32, b: f32) -> DivResult {
  let n = div(a, b)?;
  let m = div(b, a)?;
  let s = m + n;

  if s >= 10.0 {
    return Err(Box::new(AddError::Overflow("Sum is too large".to_string())));
  }

  Ok(s)
}

fn show(a: f32, b: f32)  {
    match div_and_add(a, b) {
        Ok(res) => { println!("Result: {}", res); },
        Err(e) => {
            if let Some(v) = e.downcast_ref::<DivError>() {
                match v {
                    DivError::DivisionByZero(msg) => {
                        println!("Zero division error: {}", msg);
                    }
                }
            } else if let Some(v) = e.downcast_ref::<AddError>() {
                match v {
                    AddError::Overflow(msg) => {
                        println!("Overflow error: {}", msg);
                    }
                }
            } else {
                println!("Undetermined error"); // this shouldn't happen
            }
        }
    }
}

fn main() {
    show(2.0, 4.0);   // Result: 2.5
    show(0.0, 4.0);   // Zero division error: Division by zero is illegal
    show(2.0, 0.0);   // Zero division error: Division by zero is illegal
    show(100.0, 1.0); // Overflow error: Sum is too large
}
