fn main() {
    let string: &str = "malaalam";
    let length = string.len();
    for first_loop in 0..length {
        if &string.as_bytes()[first_loop] != &string.as_bytes()[length - first_loop - 1] {
            println!("Not Palindrome");
            break;
        }
        else {
            println!("Palindrome");
            break;
        }
    }
}