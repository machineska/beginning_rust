fn main() {
    let arra = [20, 40, 80];
    update(arra);
    print!("Indise-main {:?}", arra);
}

fn update(mut arra: [i32; 3]) {
    for i in 0..3 {
        arra[i] = 0
    }
    println!("Inside-update {:?}", arra);
}
