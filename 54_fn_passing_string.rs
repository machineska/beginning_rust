fn main() {
    let name:String = String::from("RustPoint");
    display(name);
    // cannot access the name after display
}

fn display(param_name: String) {
    println!("param_name value:{}", param_name);
}
