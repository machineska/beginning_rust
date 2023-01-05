fn main() {
    let a: (i32, bool, f64) = (32, true, 7.8);
    print(a);
}

fn print(y:(i32, bool, f64)) {
    println!("print method inside");
    let (age, is_male, cgpa) = y; //assigns tuple to different vars
    println!("Age {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
}

