fn main() {
    let mut x = 5;
    { // Create a separate scope
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    } // y goes out of scope here, releasing its mutable access

    println!("x = {}", x); // x is now 6
} 