fn main() {
    let fullname = "Rust Point \r \n";
    println!("Before-trim");
    println!("lenght {}", fullname.len());
    println!("{}", fullname);
    println!();
    println!("After-trim");
    println!("length {}", fullname.trim().len());
    println!("{}", fullname);
}
