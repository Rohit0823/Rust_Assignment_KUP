/// Numbers Structure
struct Numbers {
    num_first: u32,
    num_second: u32,
}
/// This main method print the complex numbers.
///
/// #Arguments
///
/// complex of the Structure Number.
///
/// #Return
///
/// Returns successfully print all.
fn main() {
    let sum = Numbers {
        num_first: 100,
        num_second: 50.0 as u32,
    };
    println!("Addition of two numbers is {:?}+i ", sum.addition());
    println!("Subtraction of two numbers is: {:?}+i ", sum.subtract());
    println!("Multiplication of two numbers is: {:?}+i ", sum.multiplication());
    println!("Divisionn of two numbers is: {:?}+i ", sum.division());
}
/// This method calculate the two complex numbers.
///
/// #Arguments
///
/// complex of the Structure Number.
///
/// #Return
///
/// Returns result of two number.
impl Numbers {
     println!("First number and Second number is : {:?}, {:?}i", self.num_first, self.num_second);
    fn addition(&self) -> u32 {
        self.num_first + self.num_second
    }
    fn subtract(&self) -> u32 {
        self.num_first - self.num_second
    }
    fn multiplication(&self) -> u32 {
        self.num_first * self.num_second
    }
    fn division(&self) -> u32 {
        self.num_first / self.num_second
    }
}
