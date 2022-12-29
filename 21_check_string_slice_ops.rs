fn main() {
    let name: &str = "Riska";
    let slice = &name[2..];
    println!("My name is {}", slice);
}
