fn main() {
    let mut arra = [20, 40, 90];
    update(&mut arra);
    print!("Inside-main {:?}", arra);
}

fn update(arra:&mut [i32;3]) {
    for i in 0..3 {
        arra[i] = 0
    }
    println!("Inside-update {:?}", arra);
}
