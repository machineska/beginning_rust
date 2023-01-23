fn main() {
    let state_code = "DL";
    let state = match state_code {
        "DL" => {println!("Found match "); "Delhi"},
        "IN" => "India",
        "KL" => "Kolkata",
        "AB" => "Ahmadabad",
        _ => "Unknown"
    };
    println!("State name: {}", state);
}
