fn eat(count: i32, food: &str) {
    println!("You ate {} helpings of {}", count, food);
}

fn main() {
    eat(5, "apples");
    eat(8, "bananas");
}
