fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is now 6

    // This line will cause a compile-time error because z is immutable:
    //*z += 1; //This is incorrect.
}