fn main() {
    let a: (i32, bool, f64) = (120, true, 11.9);
    print(a);
}

fn print(y: (i32, bool, f64)) {
    println!("print method inside");
    println!("{:?}", y);
}
