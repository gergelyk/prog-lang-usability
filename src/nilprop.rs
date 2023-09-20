fn div(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn div_and_add(a: f32, b: f32) -> Option<f32> {
    let n = div(a, b)?;
    let m = div(b, a)?;
    let s = m + n;

    if s >= 10.0 {
        None
    } else {
        Some(s)
    }
}

fn show(a: f32, b: f32) {
    println!("{:?}", div_and_add(a, b));
}

fn main() {
    show(2.0, 4.0);   // 2.5
    show(0.0, 4.0);   // None
    show(2.0, 0.0);   // None
    show(100.0, 1.0); // None
}
