fn main() {
    let example_string1 = String::from("example string");
    print_literal(example_string1.as_str());
}

fn print_literal(data:&str){
    println!("displaying the string literal {}", data);
}
