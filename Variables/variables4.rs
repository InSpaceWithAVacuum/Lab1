// TODO: Fix the compiler error.
fn main() {
    // We need to make the variable x modifiable. So we add mut
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
