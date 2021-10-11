fn main() {
    let string: &str = "Knoldus is a tree of knowledge";
    let length = string.len();
    let mut count: i32;
    let mut char_vec: Vec<char> = string.chars().collect();

    for first_loop in 0..length {
        count = 1;
        for second_loop in first_loop + 1..length {
            if char_vec[first_loop] == char_vec[second_loop] && char_vec[first_loop] != ' '  {
                count = count + 1;
                char_vec[second_loop] = '0';
            }
        }
        if count > 1 && char_vec[first_loop] != '0' {
            println!("{}", char_vec[first_loop]);
        }
    }
}