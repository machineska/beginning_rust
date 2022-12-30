fn main() {
    let msg="Rust Point has good tutorials".to_string();
    let mut x = 1;
    for token in msg.split_whitespace(){
        println!("Token {} {}", x, token);
        x+=1;
    }
}
