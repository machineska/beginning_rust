fn main() {
    let no:i32 = 7;
    mutate_no_to_zero(no);
    println!("Value of no: {}", no);

}

fn mutate_no_to_zero(mut param_no: i32) {
    param_no = param_no * 0;
    println!("param_no value:{}", param_no);
}
