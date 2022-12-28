fn main() {
    let age:u8 = 255;
    // 0 to 255 only allowed for u8
    let weight:u8 = 256; // overflow value is 0
    let height:u8 = 257; // the overflow value is 1
    let score:u8 = 258; // the overflow value is 2
    println!("age {}", age);
    println!("weight {}", weight);
    println!("height {}", height);
    println!("score {}", score);
}
