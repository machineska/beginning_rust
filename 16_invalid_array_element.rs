use std::io;
fn main() {
    let x = [11, 22, 33, 44, 55];
    println!("Enter array index.");
    let mut index=String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not number");
    let element = x[index];
    println!(
        "Value of the element at index {} is: {}",
        index, element
    );
}
