fn main() {
    let mut no:i32 = 7;
    mutate_no_to_zero(&mut no);
    println!("Value of no: {}", no);
    fn mutate_no_to_zero(param_no:&mut i32) {
        *param_no = 0; //de reference
    }
}
