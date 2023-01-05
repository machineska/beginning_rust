fn main() {
    let arra:[i32; 4] = [20, 30, 80, 50];
    println!("array {:?}", arra);
    println!("array size :{}", arra.len());
    for index in 0..4 {
        println!("index is: {} & value: {}", index, arra[index]);
    }
}


