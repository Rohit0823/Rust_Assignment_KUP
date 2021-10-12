struct Complex {
    real: f64,
    imaginary: f64,
}

fn main() {
    let real = num::complex::Complex::new(5.0, 10.0);
    let imaginary = num::complex::Complex::new(3.1, -1.2);

    let sum = real + imaginary;
    let subtract = real - imaginary;
    let multiply = real * imaginary;

    println!("Sum: {}", sum);
    println!("Subtract: {}", subtract);
    println!("multiply: {}", multiply);
}