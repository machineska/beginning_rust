fn main() {
    let state_code = "IN";
    let state = match state_code {
        "DL" => {println!("Found Match"); "Delhi"},
        "IN" => "India",
        "KL" => "Kolkatta",
        "AB" => "Ahmadabad",
        _ => "Unknown"
    };
    println!("State name: {}", state);
}
