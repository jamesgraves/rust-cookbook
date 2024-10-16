use approx::assert_ulps_eq;

fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    println!("{} {}", a, b);
    assert_ulps_eq!(a, b);

    /*
     * Standard equality also works for x = 6.0, but may not for other values.
     */
    //  assert_eq!(a, b);
}
