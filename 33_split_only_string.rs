fn main() {
    let fullname = "Canan,Rudhasharan,Rustpoint";
    for token in fullname.split(","){
        println!("token is {}", token);
    }
    // the store in a vector
    println!("\n");
    let tokens:Vec<&str> = fullname.split(",").collect();
    println!("firstName {}", tokens[0]);
    println!("lastName {}", tokens[1]);
    println!("company {}", tokens[2]);
}

