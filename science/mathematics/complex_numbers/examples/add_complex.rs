use num_complex::Complex;

fn main() {
    let complex_num1 = Complex::new(10.0, 20.0); // Compiler will infer numeric type f32.
    let complex_num2 = Complex::new(3.1, -4.2_f32);  // Numeric type can also be specified explicitly.

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}
