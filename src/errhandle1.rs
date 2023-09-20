fn div(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        return Err("Division by zero is illegal");
    }
    Ok(a / b)
}

fn div_and_add(a: f32, b: f32) -> Result<f32, &'static str> {
    let n = div(a, b)?;
    let m = div(b, a)?;
    let s = m + n;

    if s >= 10.0 {
        return Err("Sum is too large")
    }

    Ok(s)
}

fn show(a: f32, b: f32)  {

    fn disp_res(x : &dyn std::fmt::Display ) {
        println!("Result: {}", x);
    }

    fn disp_err(x : &dyn std::fmt::Display ) {
        println!("Error: {}", x);
    }
    
    match div_and_add(a, b) {
        Ok(res) => { disp_res(&res); },
        Err(msg) => { disp_err(&msg); }
    };
}

fn main() {
    show(2.0, 4.0);   // Result: 2.5
    show(0.0, 4.0);   // Error: Division by zero is illegal
    show(2.0, 0.0);   // Error: Division by zero is illegal
    show(100.0, 1.0); // Error: Sum is too large
}
