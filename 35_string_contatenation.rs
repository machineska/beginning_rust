fn main() {
    let nm1 = "Rust".to_string();
    let nm2 = "Points".to_string();
    let nm3 = nm1 + &nm2; // n2 reference is passed
    println!("{}", nm3);
}
