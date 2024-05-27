fn temperature() -> i32 {
    15 // feel free to change this!
}

fn is_hot() -> bool {
    temperature() > 30
}

fn main() {
    assert!(is_hot() == false);
    println!("Success!");
}
