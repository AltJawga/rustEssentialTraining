fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;

    println!("Unformatted:");
    println!("c is {}", c);

    println!("Three Decimal Points:");
    println!("c is {:.3}", c);

    println!("Padding:");
    println!("c is {:8.3}", c);

    println!("Padding (Zeroes):");
    println!("c is {:08.3}", c);

    println!("Multiple Variables:");
    println!("c is {:08.3}\na is {}", c, a);

    print!("No New Line: ");
    println!("c is {:08.3}, a is {}", c, a);

    println!("Targeting Variables:");
    println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a)
}

