fn main() {
    let fullname = "Canan,Rudhasharan,Rustpoint";
    for token in fullname.split(",") {
        println!("token is {}", token);
    }
    // the store in vector
    println!("\n");
    let tokens:Vec<&str> = fullname.split(",").collect();
    println!("firstname {}", tokens[0]);
    println!("lastname {}", tokens[1]);
    println!("company {}", tokens[2]);
}
