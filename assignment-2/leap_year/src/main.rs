fn main() {
    let years_arr: [i32; 8] = [2000,2001,2012,2003,2007,2008,2020,2010];
    let mut count = 0;
    for i in 0..8 {
        if years_arr[i] % 4 == 0 {
            if years_arr[i] % 100 == 0 {
                if years_arr[i] % 400 == 0 {
                    println!("{} is leap year", years_arr[i]);
                    count = count + 1;
                }
                else {
                    println!("{} is not leap year", years_arr[i]);
                }
            }
            else {
                println!("{} is leap year", years_arr[i]);
                count = count + 1;
            }
        }
        else {
            println!("{} is not leap year", years_arr[i]);
        }
    }
    println!("Count of Leap Years is: {}", count);
}