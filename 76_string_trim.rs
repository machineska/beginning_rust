fn main() {
    let fullname = " Rust Point \r \n";
    println!("before-trim");
    println!("length {}", fullname.len());
    println!();
    println!("After-trim");
    println!("length {}", fullname.trim().len());
}
