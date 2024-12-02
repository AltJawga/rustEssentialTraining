fn main() {
    let a = 10;
    let b = 3;

    // Addition
    let mut c = a + b;
    println!("c is {}", c);

    // Subtraction
    c = a - b;
    println!("c is {}", c);

    // Multiplication
    c = a * b;
    println!("c is {}", c);

    // Divison (without remainder)
    c = a / b;
    println!("c is {}", c);

    // Remainder from division (Modulo operator)
    c = a % b;
    println!("c is {}", c);

    // Floating point division & casting
    let d = 10;
    let e = 3.0;

    let f = d as f64 / e;
    println!("f is {}", f);
}
