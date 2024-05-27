fn main() {
    let mut x = 5;

    let y = &mut x; // freeze
    *y *= 2; // unfreeze

    println!("x == {}", x);
}
