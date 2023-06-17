fn main() {
    let arra:[i32; 3] = [20, 30, 80];
    update(arra);
    print!("Inside-main {:?}", arra);
}

fn update(mut axx:[i32;3]){
    for i in 0..3 {
        axx[i] = 0;
    }
    println!("Inside-update {:?}", axx);
}
