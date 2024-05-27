fn fact(x: u32) -> u32 {
    if x == 0 {
        1
    } else {
        x * fact(x - 1)
    }
}

fn main() {
    println!("Fact of 5: {}", fact(5));
}
